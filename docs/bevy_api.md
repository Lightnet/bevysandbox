

# API
  create entity will have frame delay.




## register_type
```rust
app.register_type<name>();
app.register_type<Tower>();
```


# despawn:

```rust
	//				number id - remove all 
	commands.entity(entity).despawn_recursive();
```


```rust
	.insert(component::Name::new("Ground"))//nope wrong code
	
	.insert(Name::new(format!("{:?}_Tower", tower_type)))
```
 * https://www.youtube.com/watch?v=VSnhfm00Vz4


# Commands:
 * https://www.youtube.com/watch?v=_uKWIYEGBjs

```rust
commands.spawn();
```

Methods
```
.add
.entity
.get_or_spawn
.init_resouce
.intert_or_spawn_batch
.insert_resource
.new
.new_from_entities
.remove_resource
.spawn
.spawn_batch
.spawn_bundle
```

# EntityCommands:


Methods
```
commands
despawn
id
insert
insert_bundle
log_components
remove
remove_bundle
```

```rust
#[deruve(Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub stuct Entity{
  pub(crate) generation:u32,
  pub(crate) id:u32,
}
```

# App:
```rust
.add_startup_system() //run once
.insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2))) //background
.add_plugins(DefaultPlugins) //default set up
.run()// run app
```



```rust
commands
	.spawn_bundle(PointLightBundle{
		point_light:PointLight{
			intensity: 15000.0,
			shadows_enbled:true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	})
	.insert(component::Name::new("Light"))
```