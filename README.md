# Fuzz

A simple static site generator with markdown rendering live-reloading.
Based on https://kerkour.com/rust-static-site-generator, with ChatGPT assistance.

Install rust/cargo with [rustup](https://www.rust-lang.org/learn/get-started).
Run `cargo run serve` to build and serve the website locally, or `cargo run` just to build it.

Markdown files in /content are converted to html and written to /public.
Other file types are just copied over. You can customise this behaviour in `rebuild_site`.

The built site is served to http://127.0.0.1:8080.
Everything is live-reloaded, so changes to files in /content will rebuild the site on save, and then changes inside the public folder trigger the browser to reload. See `content_watcher` and `public_watcher` in `main.rs`.

# Why?
This is a starting point for making a static website.
In some ways it is easier to use github pages with Jekyll to do the same thing, but if you want to be able to build and test the website locally, installing Ruby/Gem and Jekyll is a pain, and it doesn't trigger browser reload by default.
By starting as simple as possible, you have more control and awareness of how the website is built.
So the best way to proceed is to read all of the code, and change it as needed for your use case.
It is *not* a black-box.

Why Rust? In my experience so far, package management has been a lot more straight-forward and less error-prone than pip or gem.
You could re-create this in another language, as long you have libraries for:
- markdown to html rendering
- file watching (for live-reloading)
- something to serve the website locally
- a way to trigger browser reload

# TODO (for you)
- Deployment: There should be a github action that builds the public folder, and then tells github pages to serve it
- Templating: `template.rs` is a barebones version of templating with something like [Sailfish](https://github.com/rust-sailfish/sailfish)
- CSS: Make it pretty
- Make the rebuilding smarter (it seems to rebuild multiple times per save)
- LaTeX rendering