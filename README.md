# Bevy Sandbox

# License: MIT

# Created By: Lightnet

# Status:
 * Prototyping
 * Work In Progress and subject to files and changes.

# Tool Chain:
 * Rust 1.76.0

# Code Language:
 * Rust Language

# Game Engine:
 * Bevy Engine 0.12.1

# Packages:
 * Bevy 0.12.1
 * bevy_mod_picking 0.14.0 (not working)
 * bevy_mod_raycast 0.16.0 (???)
 * bevy_rapier3d 0.24.0 (???)
 * ...
 * clap 4.0.32
 * bevy_egui 0.18.0


# Information:
  Note this is prototype build from Bevy Engine in Rust Language.
  
  It work in progress that subject to changes. Bevy Engine API subject to change and break.

  The ideas base on godot for single small file to run scripts. But the network required more coding to handle ID object lists.

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