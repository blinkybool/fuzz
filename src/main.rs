use axum::Router;
use std::{fs, io::{self, Write}, net::SocketAddr, path::Path};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use notify::{Watcher, RecommendedWatcher};

mod templates;
const CONTENT_DIR: &str = "content";
const PUBLIC_DIR: &str = "public";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	rebuild_site().expect("Rebuilding site");

	let livereload = LiveReloadLayer::new();
	let reloader = livereload.reloader();
	let app = Router::new()
		.nest_service("/", ServeDir::new(PUBLIC_DIR))
		.layer(livereload);

	let mut content_watcher = RecommendedWatcher::new(move |_| {
		rebuild_site().expect("Rebuilding site")
	}, notify::Config::default())?;
	content_watcher.watch(Path::new(CONTENT_DIR), notify::RecursiveMode::Recursive)?;

	let mut public_watcher = RecommendedWatcher::new(move |_| reloader.reload(), notify::Config::default())?;
	public_watcher.watch(Path::new(PUBLIC_DIR), notify::RecursiveMode::Recursive)?;

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("Serving site at http://{}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await?;

	Ok(())
}

fn rebuild_site() -> Result<(), anyhow::Error> {
	print!("rebuilding...");
	io::stdout().lock().flush().unwrap();

	let _ = fs::remove_dir_all(PUBLIC_DIR);
	let _ = fs::create_dir_all(Path::new(PUBLIC_DIR));

	let markdown_files: Vec<String> = walkdir::WalkDir::new(CONTENT_DIR)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.path().display().to_string().ends_with(".md"))
		.map(|e| e.path().display().to_string())
		.collect();
	let mut html_files = Vec::with_capacity(markdown_files.len());

	for file in &markdown_files {
		let mut html = templates::HEADER.to_owned();
		let markdown = fs::read_to_string(&file)?;
		let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());

		let mut body = String::new();
		pulldown_cmark::html::push_html(&mut body, parser);

		html.push_str(templates::render_body(&body).as_str());
		html.push_str(templates::FOOTER);

		let html_file = file
			.replace(CONTENT_DIR, PUBLIC_DIR)
			.replace(".md", ".html");
		fs::write(&html_file, html)?;

		html_files.push(html_file);
	}

	write_index(html_files, PUBLIC_DIR)?;

	// Just copy over non-markdown stuff
	for entry in walkdir::WalkDir::new(CONTENT_DIR) {
		let entry = entry?;
		let path = entry.path();
		let path_str = entry.path().display().to_string();
		if entry.path().is_file() && !path_str.ends_with(".md") {
			
			let dest_path = path_str
				.replace(CONTENT_DIR, PUBLIC_DIR);

			// Using fs::copy modifies the file metadata, triggering another build
			// So we read, then write, instead
			let file_content = fs::read_to_string(path)?;
			fs::write(dest_path, file_content)?;
		}
	}
	
	print!("done\n");
	Ok(())
}

fn write_index(files: Vec<String>, output_dir: &str) -> Result<(), anyhow::Error> {
	let mut html = templates::HEADER.to_owned();
	let body = files
		.into_iter()
		.map(|file| {
			let file = file.trim_start_matches(output_dir);
			let title = file.trim_start_matches("/").trim_end_matches(".html");
			format!(r#"<a href="{}">{}</a>"#, file, title)
		})
		.collect::<Vec<String>>()
		.join("<br />\n");

	html.push_str(templates::render_body(&body).as_str());
	html.push_str(templates::FOOTER);

	let index_path = Path::new(&output_dir).join("index.html");
	fs::write(index_path, html)?;
	Ok(())
}