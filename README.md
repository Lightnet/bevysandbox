# Bevy Sandbox

# License: MIT

# Created By: Lightnet

# Status:
 * Prototyping
 * Work In Progress and subject to files and changes.

# Tool Chain:
 * Rust 1.69.0

# Code Language:
 * Rust Language

# Game Engine:
 * Bevy Engine 0.10.1

# Packages:
 * Bevy 0.9.1
 * clap 4.0.32
 * bevy_egui 0.18.0
 * bevy_mod_picking 0.11.0
 * 

# Information:
  Note this is prototype build from Bevy Engine in Rust Language.
  
  It work in progress that subject to changes. Bevy Engine API subject to change and break.

  This is just test build ideas.

  Note it develop by users who like using rust or game engine. There are many plugins and helper scripts. It very module builds.

  * https://bevyengine.org/assets/

# Command Lines and Tests:

```
cargo run // test
cargo run --bin editor // test
cargo run --bin game // test
```

```
cargo run --example launcher // test
cargo run --example console // test
cargo run --example firstperson // test
cargo run --example menutest // test
cargo run --example physics // test
cargo run --example scripting //for lua script test
cargo run --example launcher // test
```

# Network test:
 * Need to config this later.
```
cargo run --bin network -- server
cargo run --bin network -- client
```

## Notes:
 * Examples and samples from bevyengine github for testing.
 * There are other example from other github and users from bevy community.
```
cargo run -p game --release
```
  For release stable application.

# Refs / Links:
 * https://bevyengine.org/