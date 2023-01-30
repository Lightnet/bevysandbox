# system:
  Bevy Engine design base module system.

  Architect design base on logic set up.

 Can be found in bevy engine site. Still rework some notes.

# links:
 * https://docs.rs/bevy/latest/bevy/app/struct.App.html
 * 

 * https://docs.rs/bevy/latest/bevy/app/enum.CoreStage.html
```rust
pub enum CoreStage {
  First,
  PreUpdate,
  Update,
  PostUpdate,
  Last,
}
```
  To build the game there some set up for game logic to run.

```rust
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
  MainMenu,
  GamePlay,
}
state = GameState::MainMenu
state = GameState::GamePlay
  SystemSet::on_enter(state)
  SystemSet::on_pause(state)
  SystemSet::on_update(state)
  SystemSet::on_resume(state)
  SystemSet::on_exit(state)
```

## init:
 * set up window / server
 * load assets
 * set up veriables
 * scenes
 * object
 * game enum state
 * menu ui
 
## types:
* set up 
* loop
* query
* plugin
* resources
* input
* windows
* state
* structs / components
* stageless (0.10)

  They are break up into modules and queries base types. As they can be group together by stage to deal with condition when it need to update or not by state.

```
let mut app = App::new();
```

```
app.insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
```
  Can be set up for res like variables.


```
app.add_plugin() //single
app.add_plugins() //group
```
  For set up example window DefaultPlugins.
  
```
  add_system_set(  
    SystemSet::on_enter(GameState::MainMenu)
      .with_system(setup_button01)
    )
```
  It can be group by state.
