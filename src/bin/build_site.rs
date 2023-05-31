extern crate fuzz;
use anyhow::Ok;
use fuzz::build_site;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		panic!("This program requires exactly two arguments.");
	}

	let base_url = &args[1];
	build_site(base_url).unwrap();
}