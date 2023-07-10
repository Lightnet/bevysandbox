
# CHEAT BOOK:
 * https://bevy-cheatbook.github.io/
 * https://github.com/jakobhellermann/bevy-inspector-egui
 * https://bevyengine.org/learn/book/getting-started/setup/


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


# Resource
 * https://bevyengine.org/examples/games/breakout/
```rust
// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

app.insert_resource(Scoreboard { score: 0 })
app.add_system(update_scoreboard)


fn check_for_collisions(
  
  mut commands: Commands,
  mut scoreboard: ResMut<Scoreboard>,
  //..
) {
    // Bricks should be despawned and increment the scoreboard on collision
    if maybe_brick.is_some() {
        scoreboard.score += 1;
        commands.entity(collider_entity).despawn();
    }

}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
  let mut text = query.single_mut();
  text.sections[1].value = scoreboard.score.to_string();
}
```

https://bevy-cheatbook.github.io/features/parent-child.html

```rust
// spawn the parent and get its Entity id
let parent = commands.spawn(MyParentBundle::default()).id();

// do the same for the child
let child = commands.spawn(MyChildBundle::default()).id();

// add the child to the parent
commands.entity(parent).push_children(&[child]);

// you can also use `with_children`:
commands.spawn(MyParentBundle::default())
    .with_children(|parent| {
        parent.spawn(MyChildBundle::default());
    });
```

```rust
fn close_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in query.iter() {
        // despawn the entity and its children
        commands.entity(entity).despawn_recursive();
    }
}
```

https://www.youtube.com/watch?v=_tBR0KckRMQ



