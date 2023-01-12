pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
  fn build(&self, app:&mut App){
    app.add_system_set(SystenSet::on_enter(GameState::MainMenu).with_system(spawn_main_menu))
  }
}

fn spawn_main_menu(mut commands:Commands, asset_server:Res<AssetServer>){

}