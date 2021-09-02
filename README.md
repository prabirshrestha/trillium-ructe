trillium-ructe
==============

[ructe](https://crates.io/crates/ructe) templates for [trillium.rs](https://trillium.rs/).

# Getting Started

```toml
[package]
name = "example-ructe-template"
edition = "2018"
build = "src/build.rs"

[dependencies]
trillium = "0.3.0"
trillium-ructe = "0.3.0"
trillium-smol = "0.3.0"

[build-dependencies]
ructe = { version = "0.13.4", features = ["sass"] }
```

# Example

## src/build.rs

```rust
use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("static")?;
    statics.add_sass_file("styles/style.scss")?;
    ructe.compile_templates("templates")?;
    Ok(())
}
```

## templates/helloworld.rs.html

```html
@(text: &str)
<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Trillium Ructe Example</title>
    </head>
    <body>
        <h1>@text</h1>
    </body>
</html>
```

## src/main.rs

```rust
use trillium::Conn;
use trillium_ructe::RucteConnExt;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

fn main() {
    trillium_smol::run(|conn: Conn| async move {
        conn.render_html(|o| templates::helloworld(o, "html"))
    });
}
```

Use `conn.render_html` to render html or `conn.render` to render raw template.
