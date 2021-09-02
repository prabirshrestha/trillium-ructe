#![forbid(unsafe_code)]
use std::io::Write;
use trillium::{Conn, KnownHeaderName::ContentType};

/**
Renders a template and sets content-type as "text/html; charset=utf-8" or returns the conn with a 500 status.
```ignore
use trillium::Conn;
use trillium_ructe::RucteConnExt;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

fn main() {
    trillium_smol::run(|conn: Conn| async move {
        // helloworld.rs.html contents:
        //  @(text: &str)
        //  <h1>@text</h1>
        conn.render_html(|o| templates::helloworld(o, "hello world"))
    });
}

```
*/

pub trait RucteConnExt {
    /// Render a ructe template to this conn's body.
    ///
    /// Allocates a default buffer size of 1kb
    fn render<F>(self, render_fn: F) -> Self
    where
        F: FnOnce(&mut dyn Write) -> std::io::Result<()>;

    /// Render a ructe template to this conn's body, starting with an
    /// allocated buffer of the supplied size in bytes.
    fn render_with_size_estimate<F>(self, render_fn: F, size_estimate: usize) -> Self
    where
        F: FnOnce(&mut dyn Write) -> std::io::Result<()>;

    /// Render a ructe template to this conn's body and set a content
    /// type header of text/html.
    ///
    /// Allocates a default buffer size of 1kb.
    fn render_html<F>(self, render_fn: F) -> Self
    where
        F: FnOnce(&mut dyn Write) -> std::io::Result<()>;
}

impl RucteConnExt for Conn {
    fn render<F>(self, render_fn: F) -> Self
    where
        F: FnOnce(&mut dyn Write) -> std::io::Result<()>,
    {
        self.render_with_size_estimate(render_fn, 1024)
    }

    fn render_html<F>(self, render_fn: F) -> Self
    where
        F: FnOnce(&mut dyn Write) -> std::io::Result<()>,
    {
        self.render(render_fn)
            .with_header(ContentType, "text/html; charset=utf-8")
    }

    fn render_with_size_estimate<F>(self, render_fn: F, size_estimate: usize) -> Self
    where
        F: FnOnce(&mut dyn Write) -> std::io::Result<()>,
    {
        let mut body = Vec::with_capacity(size_estimate);
        trillium::conn_try!(render_fn(&mut body), self);
        self.ok(body)
    }
}
