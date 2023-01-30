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

# Information:
  Note this is prototype build from Bevy Engine in Rust Language Code. This work space set up build for testing applications.

  To develop editor, debug, game and module design is code in rust Language.

  The idea is base on Bevy engine has some basic information and examples. Idea base or simalar to godot engine. By node tree, components, res and script.

  You can create components and loop logic. It use query call components to filter the update call when it match and will update the entities component.

  The entity build on components. But required some default config base on entity types.

```
entity
-transform (position, rotation, scale)
-player (component )
-player_movement (function loop query)
```

fn player_movement
```
(
param > player tag component,
param > input tag component,
)
{
  // do something
}
```

Query component for player tag filter. It would check if the entity component name matches player tag. It would update movement. But it would required input query as well. There are example game in Bevy engine.

There is build in function to easy of access.


# Design:
  To build simple minecraft block but limited in sandbox ways by building simple first.

# Command Lines (work space):
```
cargo run --package console
cargo run --package editor
cargo run --package example01
cargo run --package firstperson
cargo run --package game
cargo run --package menutest
cargo run --package network
cargo run --package physics
cargo run --package scripting //for lua script test
cargo run --package shared
```
Since it work space you need to run some more command args.

```
cargo run --package game --release
```
  For release stable application.

```
cargo run --example test
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