use axum::Router;
use std::{io::{self, Write}, net::SocketAddr, path::Path};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use notify::{Watcher, RecommendedWatcher};
use fuzz::build_site;

const CONTENT_DIR: &str = "content";
const BUILD_DIR: &str = "_site";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	build_site("/", BUILD_DIR).expect("Failed to build site");

	let livereload = LiveReloadLayer::new();
	let reloader = livereload.reloader();
	let app = Router::new()
		.nest_service("/", ServeDir::new(BUILD_DIR))
		.layer(livereload);

	let mut content_watcher = RecommendedWatcher::new(move |_| {
		print!("rebuilding...");
		io::stdout().lock().flush().unwrap();
		build_site("/", BUILD_DIR).expect("Failed to rebuild site");
		print!("done\n");
	}, notify::Config::default())?;
	content_watcher.watch(Path::new(CONTENT_DIR), notify::RecursiveMode::Recursive)?;

	let mut site_watcher = RecommendedWatcher::new(move |_| reloader.reload(), notify::Config::default())?;
	site_watcher.watch(Path::new(BUILD_DIR), notify::RecursiveMode::Recursive)?;

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("Serving site at http://{}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await?;

	Ok(())
}