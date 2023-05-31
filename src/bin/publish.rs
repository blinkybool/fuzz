use std::process::Command;
use std::{fs};
use std::path::Path;
use anyhow::Ok;

use fuzz::build_site;

const BASE_URL: &str = "/fuzz/";

fn main() -> Result<(), anyhow::Error> {
	// Create git worktree for gh-pages branch inside the repo
	let worktrees = Command::new("git")
		.args(["worktree", "list"])
		.output()?;

	if !String::from_utf8_lossy(&worktrees.stdout).contains("gh-pages") {
		Command::new("git")
			.args(["worktree", "add", "gh-pages"])
			.output()?;
	}

	// Remove the existing site content
	for entry in fs::read_dir("gh-pages")? {
		let path = entry?.path();
		if !path.ends_with(".git") {
			fs::remove_dir_all(path.clone()).unwrap_or_else(|_| fs::remove_file(path).unwrap());
		}
	}

	build_site(BASE_URL, "gh-pages/_site").expect("Failed to build site.");

	// Move gh-pages/_site contents to gh-pages
	for entry in fs::read_dir("gh-pages/_site")? {
		let path = entry?.path();
		if !path.ends_with(".git") {
			fs::rename(path.clone(), Path::new("gh-pages").join(path.file_name().unwrap()))?;
		}
	}

	fs::remove_dir("gh-pages/_site").expect("Failed to remove gh-pages/_site");
	fs::write("gh-pages/.nojekyll", "").expect("Failed to write .nojekyll file");

	let output = Command::new("git")
		.args(["-C", "./gh-pages", "diff"])
    .output()?;

	if output.stdout.is_empty() {
		println!("no unstaged changes in gh-pages branch");
		return Ok(());
	}

	Command::new("git")
		.args(["-C", "./gh-pages", "add", "-A"])
		.output()?;

	Command::new("git")
		.args(["-C", "./gh-pages", "commit", "-m", "\"Publish site\""])
		.output()?;

	Command::new("git")
		.args(["-C", "./gh-pages", "push", "origin", "gh-pages"])
		.output()?;

	let output = Command::new("git")
		.args(["-C", "./gh-pages", "log", "-1", "--stat"])
		.output()?;

	println!("Published changes to gh-pages branch\n{}", String::from_utf8_lossy(&output.stdout));

	Ok(())
}
