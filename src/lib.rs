#![forbid(unsafe_code)]

use std::io::Write;
use trillium::{Conn, KnownHeaderName::ContentType};

#[derive(thiserror::Error, Debug)]
pub enum RucteError {
    #[error("Failed to render ructe template")]
    IoError {
        #[source]
        source: std::io::Error,
        conn: Conn,
    },
}

pub fn render<F>(call: F, conn: Conn) -> Result<Conn, RucteError>
where
    F: FnOnce(&mut dyn Write) -> std::io::Result<()>,
{
    let mut buf = Vec::new();
    match call(&mut buf) {
        Ok(()) => Ok(conn.ok(buf)),
        Err(source) => Err(RucteError::IoError { source, conn }),
    }
}

pub fn render_html<F>(call: F, conn: Conn) -> Result<Conn, RucteError>
where
    F: FnOnce(&mut dyn Write) -> std::io::Result<()>,
{
    render(
        call,
        conn.with_header(ContentType, "text/html; charset=utf-8"),
    )
}

/**
Renders a template or returns the conn with a 500 status.

```ignore
use trillium::Conn;
use trillium_ructe::render_html_try;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

fn main() {
    trillium_smol::run(|conn: Conn| async move {
        // helloworld.rs.html contents:
        //  @(text: &str)
        //  <h1>@text</h1>
        render_try!(|o| templates::helloworld(o, "hello world"), conn)
    });
}
```
*/
#[macro_export]
macro_rules! render_try {
    ($expr:expr, $conn:expr) => {
        match $crate::render($expr, $conn) {
            Ok(conn) => conn,
            Err(e) => match e {
                $crate::RucteError::IoError { source, conn } => {
                    trillium::log::error!("{}:{} render_try error: {}", file!(), line!(), source);
                    return conn.with_status(500).halt();
                }
            },
        }
    };
}

/**
Renders a template and sets content-type as "text/html; charset=utf-8" or returns the conn with a 500 status.
```ignore
use trillium::Conn;
use trillium_ructe::render_html_try;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

fn main() {
    trillium_smol::run(|conn: Conn| async move {
        // helloworld.rs.html contents:
        //  @(text: &str)
        //  <h1>@text</h1>
        render_html_try!(|o| templates::helloworld(o, "hello world"), conn)
    });
}

```
*/
#[macro_export]
macro_rules! render_html_try {
    ($expr:expr, $conn:expr) => {
        match $crate::render_html($expr, $conn) {
            Ok(conn) => conn,
            Err(e) => match e {
                $crate::RucteError::IoError { source, conn } => {
                    trillium::log::error!(
                        "{}:{} render_html_try error: {}",
                        file!(),
                        line!(),
                        source
                    );
                    return conn.with_status(500).halt();
                }
            },
        }
    };
}
