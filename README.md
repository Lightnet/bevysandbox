# Bevy Sandbox

# License: MIT

# tool chain:
 * Rust

# Code Language:
 * Rust Language

# Status:
 * Bevy Engine ( infancy prototyping phase)
 * prototyping

# Information:

  Testing Bevy Engine builds.

  This work space set up build for testing application.

  Trying to see if possible to learn to build some fun and simple games on Bevy Engine logic, features and layout design.

# Design:
  Testing some idea on building some creation format with the engine, script and code to build live code client and server. But it will be layer the basic net code.

  Idea base on game master build the world live. By using the peer to peer object checks as well block data.

  Need some predefined material and objects.

```
objectID: hash id (server/client id mapping)
hash: hash data object (check for data hash change that might need to be update.)
-mesh 
-terrain
-physics
-events
-network
-change in state
```
  One reason is the change in data if the bandwidth is limited. As well other things.

# Editor / Permission:
 Create editor required some thinking. 
 
 One is static and dynamic map. Simple reason if the player wanted to create story world required testing. Another is simple death match game need to test it logic. To able to create some instance or battle arena. Need to create some respawn point and death checks for condition for restart or game over.
 
 Two for editig map for event and triggers.

 Undo function and time line edit.

 Player actions permission.

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
 * 
 * 
 * 
