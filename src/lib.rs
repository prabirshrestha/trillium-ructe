#![forbid(unsafe_code)]

use std::io::{Result, Write};
use trillium::Conn;

pub trait RucteConnExt {
    fn render<F>(self, do_render: F) -> Result<Self>
    where
        Self: Sized,
        F: FnOnce(&mut dyn Write) -> Result<()>;

    fn render_html<F>(self, do_render: F) -> Result<Self>
    where
        Self: Sized,
        F: FnOnce(&mut dyn Write) -> Result<()>;
}

impl RucteConnExt for Conn {
    fn render<F>(self, do_render: F) -> Result<Self>
    where
        Self: Sized,
        F: FnOnce(&mut dyn Write) -> Result<()>,
    {
        let mut buf = Vec::new();
        match do_render(&mut buf) {
            Ok(()) => Ok(self.ok(buf)),
            Err(e) => Err(e),
        }
    }

    fn render_html<F>(self, do_render: F) -> Result<Self>
    where
        Self: Sized,
        F: FnOnce(&mut dyn Write) -> Result<()>,
    {
        Ok(self
            .with_header(("content-type", "text/html; charset=utf-8"))
            .render(do_render)?)
    }
}
