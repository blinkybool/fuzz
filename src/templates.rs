pub const HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">

	<head>
		<meta charset="utf-8">
		<meta name="viewport" content="width=device-width, initial-scale=1">
		<link rel="stylesheet" href="styles.css" />
	</head>

"#;

pub fn render_body(body: &str) -> String {
	format!(r#" 
	<body>
		<nav>
			<a href="/">Home</a>
		</nav>
		<br />
		<article class="post">
		{}
		</article>
	</body>"#,
	body)
}

pub const FOOTER: &str = r#"
</html>
"#;