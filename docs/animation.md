https://bevyengine.org/examples/Animation/gltf-skinned-mesh/

```rust
let mesh_node_children = children_query.get(mesh_node_entity).unwrap();
```
 * https://www.reddit.com/r/bevy/comments/14564sy/getting_the_transform_of_a_bone_by_name_but_only/


 * https://discord.com/channels/691052431525675048/1003974538247278702/1003990772997173278
 * https://discord.com/channels/691052431525675048/774027865020039209/947812043182981130
 * 
 * https://github.com/HouraiTeahouse/bevy_prototype_animation/blob/efc4f7a0ade604694f3b3703a4f506c65f071837/src/graph/hierarchy.rs
 * 
 * 
 * 
 ```
 I have an external crate (linked above) that has a prototype implementation of a Reflect-based property animation system. It shares a lot of similarities with the big animation PR, but is usable as an external crate. There's only two things missing from it's implementation and one thing that needs to be checked in:
 1. Animation bone bindings need to have a system to match up bones with the entities. There's already a system to apply animations to bound bones, but I need a system to bind the bones to the animation graph.
 2. A converter from GLTF loaded animations to skeletal animation curves is needed, since the system supports any arbitrary component: not just skeletal animation.
 3. The mesh skinning PR from earlier needs to have some 0.6 compatible version  merged into bevy_render. 
[4:37 AM]james7132: #1 I can get done soon. I just need to find a workable solution.
#2 is going to need some work. But we can have a work around solution in the meantime.
#3 @Nguyễn Đức Long said they would take a stab at it near the end of this week. 

 ```