const CONTENT_DIR: &str = "content";

pub fn build_site(base_url: &str, output_dir: &str) -> Result<(), anyhow::Error> {
	let _ = std::fs::remove_dir_all(output_dir);
	let _ = std::fs::create_dir_all(output_dir);

	let markdown_files: Vec<String> = walkdir::WalkDir::new(CONTENT_DIR)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.path().display().to_string().ends_with(".md"))
		.map(|e| e.path().display().to_string())
		.collect();
	let mut html_files = Vec::with_capacity(markdown_files.len());

	for file in &markdown_files {
		let mut html = render_header(base_url);
		let markdown = std::fs::read_to_string(&file)?;
		let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());

		let mut body = String::new();
		pulldown_cmark::html::push_html(&mut body, parser);

		html.push_str(render_body(&body).as_str());
		html.push_str(FOOTER);

		let html_file = file
			.replace(CONTENT_DIR, output_dir)
			.replace(".md", ".html");
		std::fs::write(&html_file, html)?;

		html_files.push(html_file);
	}

	write_index(html_files, base_url, output_dir)?;

	// Just copy over non-markdown stuff
	for entry in walkdir::WalkDir::new(CONTENT_DIR) {
		let entry = entry?;
		let path = entry.path();
		let path_str = entry.path().display().to_string();
		if entry.path().is_file() && !path_str.ends_with(".md") {
			
			let dest_path = path_str.replace(CONTENT_DIR, output_dir);

			// Using fs::copy modifies the file metadata, triggering another build
			// So we read, then write, instead
			let file_content = std::fs::read_to_string(path)?;
			std::fs::write(dest_path, file_content)?;
		}
	}

	Ok(())
}

// Create index.html file with links to all markdown files
// It's unlikely you actually want a link to every markdown page in the header
// so you should change this with your own logic.
fn write_index(files: Vec<String>, base_url: &str, output_dir: &str) -> Result<(), anyhow::Error> {
	let mut html = render_header(base_url);
	let body = files
		.into_iter()
		.map(|file| {
			let file = file.trim_start_matches(output_dir);
			let title = file.trim_start_matches("/").trim_end_matches(".html");
			format!(r#"<a href=".{}">{}</a>"#, file, title)
		})
		.collect::<Vec<String>>()
		.join("<br />\n");

	html.push_str(render_body(&body).as_str());
	html.push_str(FOOTER);

	let index_path = std::path::Path::new(&output_dir).join("index.html");
	std::fs::write(index_path, html)?;
	Ok(())
}

fn render_header(base_url: &str) -> String {
	format!(r#"<!DOCTYPE html>
	<html lang="en">
	
		<head>
			<meta charset="utf-8">
			<meta name="viewport" content="width=device-width, initial-scale=1">
			<link rel="stylesheet" href="styles.css" />
			<base href="{}">
		</head>
	
	"#, base_url)
}	

fn render_body(body: &str) -> String {
	format!(r#" 
	<body>
		<nav>
			<a href=".">Home</a>
		</nav>
		<br />
		<article class="post">
		{}
		</article>
	</body>"#,
	body)
}

const FOOTER: &str = r#"
</html>
"#;