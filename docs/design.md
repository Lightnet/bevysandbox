# status:
Working on the detail of the project.

This is work space and not project. It needs for editor, game and other testing.


# Information:
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