use std::process::Command;
use std::fs;
use std::env;
use std::path::Path;

use fuzz::build_site;

const BASE_URL: &str = "/fuzz/";

fn main() {
	let current_dir = env::current_dir().unwrap();

	// Create git worktree for gh-pages branch inside the repo
	let worktrees = Command::new("git")
		.args(["worktree", "list"])
		.output()
		.expect("Failed to list worktrees");

	if !String::from_utf8_lossy(&worktrees.stdout).contains("gh-pages") {
		Command::new("git worktree add gh-pages")
			.spawn()
			.expect("Failed to list worktrees");
	}

	// Remove the existing site content
	for entry in fs::read_dir("gh-pages").unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		if path != Path::new(".git") {
			fs::remove_dir_all(entry.path()).unwrap_or_else(|_| fs::remove_file(path).unwrap());
		}
	}

	build_site(BASE_URL, "gh_pages").expect("Failed to build site.");

	// Step 3: Add the files to the repository
	let output = Command::new("git")
		.args(["-C", "./gh-pages", "add", "-A"])
		.output()
		.expect("Failed to add files to the repository");

	println!("Git add output: {}", String::from_utf8_lossy(&output.stdout));

	// Step 4: Commit the changes
	let output = Command::new("git")
	.args(["-C", "./gh-pages", "commit", "-m", "\"Publish site\""])
		.output()
		.expect("Failed to commit the changes");

	println!("Git commit output: {}", String::from_utf8_lossy(&output.stdout));

	// Step 5: Push the changes to GitHub
	let output = Command::new("git")
		.args(["-C", "./gh-pages", "push", "origin", "gh-pages"])
		.output()
		.expect("Failed to push the changes to GitHub");

	println!("Git push output: {}", String::from_utf8_lossy(&output.stdout));
}
