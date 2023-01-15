
# CHEAT BOOK:
 * https://bevy-cheatbook.github.io/
 * https://github.com/jakobhellermann/bevy-inspector-egui


https://bevyengine.org/learn/book/getting-started/ecs/


https://github.com/jakobhellermann/bevy-inspector-egui/tree/main/crates/bevy-inspector-egui/examples


https://www.youtube.com/watch?v=_uKWIYEGBjs&list=PLT_D88-MTFOPPl75g4WshL1Gx2bnGTUkz
Bevy 0.8 Intro Tutorial - A Basic 3d Scene (Ep1) (Updated to 0.9 Check Description)


# version 0.9:
```rust
commands.spawn((A, B, C));

let entity = commands.spawn_empty().id();

let entity = world.spawn_empty().id();

commands.spawn_empty().insert(SomeBundle::default());

commands.entity(some_entity).remove::<SomeBundle>();


world.entity_mut(some_entity).remove_intersection::<SomeBundle>();


// New (0.9) - Option 1
commands.spawn_empty().insert((
  SomeBundle::default(),
  SomeComponent,
))

// New (0.9) - Option 2
commands.spawn((
  SomeBundle::default(),
  SomeComponent,
))


// New (0.9)
fn some_system(world: &mut World, transforms: &mut QueryState<&Transform>) {
  for transform in transforms.iter(world) {
  }
}








```









