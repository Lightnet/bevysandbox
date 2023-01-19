# Bevy Sandbox

# License: MIT

# tool chain:
 * Rust

# Code Language:
 * Rust Language

# Status:
 * prototyping
 * Bevy Engine ( infancy prototyping phase)
 

# Packages:
 * Bevy 0.9.1
 * clap 4.0.32
 * bevy_egui 0.18.0
 * bevy_mod_picking 0.11.0
 * 
 * 

# Information:
  Note this is dev build since it an prototype phase from Bevy Engine in Rust Language Code. This work space set up build for testing applications.

  To develop editor, debug, game and module design in rust Language Code.

# Design:
  To build simple minecraft block but limited in sandbox ways by building simple first.

# Command Lines (work space):
```
cargo run --package editor --release
cargo run --package editor
cargo run --package game
cargo run --package menutest
cargo run --package gamemaster
cargo run --package physics
```
Since it work space you need to run some more command args.

```
cargo run --example test
```
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

# Command Lines(one project):
```
cargo build
```
build app
```
cargo run
```
start app

# Notes:
 * Examples and samples from bevyengine github for testing.
 * There are other example from other github and users from bevy community.

# Refs / Links:
 * https://bevyengine.org/
 * 
 * 