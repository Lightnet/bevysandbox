// https://stackoverflow.com/questions/30291757/attaching-an-icon-resource-to-a-rust-application
// https://github.com/mxre/winres
use std::{io, env};
use winres::WindowsResource;

fn main() -> io::Result<()> {
  if env::var_os("CARGO_CFG_WINDOWS").is_some() {
      WindowsResource::new()
        // This path can be absolute, or relative to your crate root.
        .set_icon("assets/icon.ico")
        .compile()?;
  }
  Ok(())
}