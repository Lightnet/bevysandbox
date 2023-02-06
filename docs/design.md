# status:
Working on the detail of the project.

This is work space and not project. It needs for editor, game and other testing.

# Information:
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




  There are some basic default plugin for window, headless and others. For preset set up application to run.

  Three basic develop for Entity, Components and system. They are break up into parts that handle the engine best possible to run game smoothly. As the users have to create their own way to handle the build and set up.

  Entity is made up empty for scene to render to place there. It use by components for later use by system query for update, inputs, render and other user code functions.

  Component tag need to be register into the bevy engine for later use like editor.

  * https://bevyengine.org/learn/book/getting-started/ecs/

  System are for update, stages, assets and other things does thing run or set up.

  Resource are for assets loading, set up struct variables and logic configs.

  You can think of Godot Engine node tree system is base on idea designs. It very module design.

  One is that you prefab or premade scene or entity from bevy engine save and load build.

  * https://github.com/bevyengine/bevy/blob/main/examples/scene/scene.rs


# Design:
  To build simple minecraft block but limited in sandbox ways by building simple first.



  Bevy Engine has module design components. It would required editor, debug and game to work correctly. 
  As well some basic components for debug tools to make sure they are visible. 
  For example is lights, camera, physics shapes and other things that are not visible for handler for transform, rotation and need hepler.

  Need to buiild some common primitive meshs, textures, materials and other things.

  Idea base on minecraft. As it can be edit live world and server and client.
  By using the peer to peer object checks as well block data.

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

# Network:
  To build editor real time is not easy. Required data transfar.

  To either have local and server is not easy task to do when doing editor mode.

# Notes:
 * bevy_egui -for some reason it only dev since it might lag render
 * bevy_egui and bevy_console have conflict that I test as it crashed.

# Bevy components:

```
  enitily
    player{
      health
      xp
      name
    }
    shape mesh
    physics
    transform
```

https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies/