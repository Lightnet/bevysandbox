# Bevy Sandbox

# License: MIT

# tool chain:
 * Rust

# Code Language:
 * Rust Language

# Status:
 * prototyping
 * Bevy Engine ( infancy prototyping phase)
 * work in progress files and changes.
 
# Packages:
 * Bevy 0.9.1
 * clap 4.0.32
 * bevy_egui 0.18.0
 * bevy_mod_picking 0.11.0

# Information:
  Note this is prototype build from Bevy Engine in Rust Language. This work space set up build for testing applications.

  This is just test build ideas. It work in progress that subject to changes.

# Command Lines (work space):
```
cargo run --package console
cargo run --package editor
cargo run --package firstperson
cargo run --package game
cargo run --package menutest
cargo run --package network
cargo run --package physics
cargo run --package scripting //for lua script test
cargo run --package launcher
```
Since it work space you need to run some more command args.

```
cargo run --package game --release
```
  For release stable application.

```
cargo run --example <name>
```
  It will check each Cargo.tomal example config tag.

Look in projects that matches the name in Cargo.toml file.
```
[[example]]
name = "test"
path = "examples/test.rs"
crate-type = ["bin"]
```

Cargo.toml
```
[package]
name = "game"
version = "0.1.0"
edition = "2021"

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[dependencies]
bevy = {version = "0.9", features = []}
bevy_console = "0.6.0"
clap = "4.0.32"
bevy_egui = "0.18.0"
```

# Notes:
 * Examples and samples from bevyengine github for testing.
 * There are other example from other github and users from bevy community.

# Refs / Links:
 * https://bevyengine.org/
 * 
 * 