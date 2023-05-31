# Fuzz

A starter-template for a simple static site with markdown rendering, live-reloading, and publishing to github pages.
Based on https://kerkour.com/rust-static-site-generator, with ChatGPT assistance.

Install rust/cargo with [rustup](https://www.rust-lang.org/learn/get-started).
Run `cargo run` to build and serve the website locally.

Markdown files in /content are converted to html and written to /_site.
Other file types are just copied over. You can customise this behaviour in `lib.rs` -> `build_site`.

The built site is served to http://127.0.0.1:8080.
Everything is live-reloaded, so changes to files in /content will rebuild the site on save, and then changes inside the /_site folder trigger the browser to reload. See `content_watcher` and `site_watcher` in `main.rs`.

# Publishing to Github Pages

I've opted to build and manually publish to the gh-pages branch, because it's less magic than github actions, which are slow because you have to compile rust and rebuild the project every single time.

What we are doing here automatically with `cargo run --bin publish` is equivalently to copying the contents of /_site to the gh-pages branch of the repo (as well as adding a blank file called ".nojekyll), and pushing the changes.

Before publishing, you must change the variable `BASE_URL` according to your needs.
For example, if your website will be at `blinkybool.github.io/fuzz/`, then `BASE_URL` should be "/fuzz/".
If you have a custom domain it will probably just be "/". This gets used in the html header of every page, so that links to pages work correctly.
Locally we use "/" as the base-url, but in the published version it might need to be something else.

To publish, run `cargo run --bin publish`. This will
- create a git worktree for the branch called `gh-pages`
- clear the existing files
- build the site
- move the site contents into the branch (worktree)
- push the changes to github

> What's a [worktree](https://git-scm.com/docs/git-worktree)? It's a way to have multiple branches checked out locally. A worktree exists by default within the main repo, but the main worktree won't confuse it for a new file to commit. You can cd into it and execute git commands as normal.

Now go to your github repo settings and then the "Pages" tab, and ensure that it's set to use
the "gh-pages" branch to deploy the website. Your changes should appear on your website shortly
(you may have to empty your browser cache if you recently visited your website).

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
- a deployment workflow (locally like I did or with a github action)

# Migration?
In theory, you could migrate your existing website, if it's simple enough, by visiting your website in the browser, and downloading the css and html files with the web-inspector. Then you just need to extract a basic html template for each kind of page/section, and decide where to inject your markdown files into those templates. CSS files can just be copied into content, like I have in this example repo.

# TODO (for you)
- [x] Deployment to github pages.
- Templating: `template.rs` is a barebones version of templating with something like [Sailfish](https://github.com/rust-sailfish/sailfish)
- CSS: Make it pretty
- Make the rebuilding smarter (it seems to rebuild multiple times per save)
- LaTeX rendering