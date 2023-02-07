Working on the detail of the project.


https://doc.rust-lang.org/cargo/reference/manifest.html

https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples


https://stackoverflow.com/questions/26946646/package-with-both-a-library-and-a-binary


Cargo.toml
```
[package]
name = "example"
version = "0.1.0"
edition = "2018"
```

src/lib.rs
```
use std::error::Error;

pub fn really_complicated_code(a: u8, b: u8) -> Result<u8, Box<dyn Error>> {
    Ok(a + b)
}
```
src/main.rs
```
fn main() {
    println!(
        "I'm using the library: {:?}",
        example::really_complicated_code(1, 2)
    );
}
```

