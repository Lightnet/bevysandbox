# Bevy Sandbox

# License: MIT

# Created By: Lightnet

# Code Language:
 * Rust Language

# Status:
 * Prototyping
 * Work In Progress and subject to files and changes.

# Tool Chain:
 * Rust 1.76.0

# Game Engine:
 * Bevy Engine 0.13.0

# Packages:
 * bevy
 * bevy_mod_raycast (?)
 * bevy_rapier3d (?)
 * bevy_egui 
 * bevy-inspector-egui
 * bevy_renet


# Information:
  Note this is prototype build from Bevy Engine in Rust Language.
  
  It work in progress that subject to changes. Bevy Engine API subject to change and break.

  The ideas base on godot for single small file to run scripts.

  To create sandbox base on block base on Minecraft game and other game mixed into as well.
  
  Network required more coding to handle ID object lists.

  * https://bevyengine.org/assets/

# Design:
 Work in progress...

 Current rework the build and sample test builds in each area.

 Just simple load player move around test. Working toward placing the block and remove block build. Swtiching block for place blooks. 
 
 Network build. Lobby?

 States:
 * DataQuery:
   * Game
   * Save
   * Load
 * network
   * offline
   * setup
   * server
   * client
   * close
   * lobby
 * app
   * main menu
   * server menu
   * client menu
   * lobbdy menu
   * assets
   * loading
   * game
   * pause
   * ingame menu
 * assets
  * idle
  * preload
  * load
  * unload
 * player
  * load
  * save
 * camera
   * menu
   * attach
   * player
   * vehicle
   * ghost
# TODOLIST:
 * HUD
    * inventory menu
    * main menu (part done)
    * settings
    * network (part done test)
    * debug (WIP)
    * hotbar
      * layout (part done)
 * save and load (simple test)
   * scene
   * player data
   * inventory
   * entities
 * character
    * data
    * animation
    * loadout
    * movement
    * inventory
    * controller
 * Game Mode
   * survival
   * creative
   * team battle
 * World
   * generate terrain
   * save and load
   * blocks
 * phyisics
   * detect
   * trigger
   * player controller
   * entity controller
 * camera
   * first person view 
   * third person view
   * specter / fly camera (part done)
 * network
   * peer to peer
   * server
   * client

# Command Lines and Tests:
 Note this is work in progress tests.
```
cargo run // test
cargo run --bin editor // test
cargo run --bin game // test
```

```
cargo run --example firstperson // test
cargo run --example menutest // test
cargo run --example physics // test
```

# Network Test ( N/A ):
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


# Credits:
 Name: kenney
 Type: Content Assets
  * www.kenney.nl