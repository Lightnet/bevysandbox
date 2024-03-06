// https://github.com/bevyengine/bevy/blob/release-0.13.1/examples/2d/texture_atlas.rs

// ASSETS
// https://kenney.nl/assets/input-prompts
// version 1.1
// ~ = Backquote , https://docs.rs/bevy/latest/bevy/prelude/enum.KeyCode.html

use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

#[derive(Component)]
struct KEY0;
// https://users.rust-lang.org/t/can-we-make-vector-of-structs-if-yes-than-how-to-make-and-use-it/19476
#[derive(Debug, Resource)]
pub struct ViewKey {
  press_id:usize,
  release_id:usize,
  key:KeyCode,
  row:usize,
}

#[derive(Component)]
pub struct TagInputKey {
  key:KeyCode
}


#[derive(Resource)]
pub struct DisplayKeys{
  key_list:Vec<ViewKey>
}

impl Default for DisplayKeys {
  fn default() -> Self {
    DisplayKeys{key_list: Vec::new()}
  }
}

fn main() {
  //let boardkeys: Vec<ViewKey> = vec![ViewKey{press_id:0,release_id:1,key:KeyCode::Numpad0}];
  //let mut mykey: KeyBoards = KeyBoards{ key_list: boardkeys };
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // fallback to nearest sampling
    .add_plugins(VKeyBoardPlugin)
    //.init_state::<AppState>()//
    //.init_resource(KeyBoards{ key_list: todo!() })
    //.init_resource(KeyBoards::default())
    //.add_systems(OnEnter(AppState::Setup), load_textures)
    //.add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
    //.add_systems(Update, keyboard_input_system.run_if(in_state(AppState::Finished)))
    //.add_systems(OnEnter(AppState::Finished), setup)
    .run();
}


pub struct VKeyBoardPlugin;

impl Plugin for VKeyBoardPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.init_state::<AppState>();
        app.insert_resource(DisplayKeys::default());
        app.add_systems(OnEnter(AppState::Setup), load_textures);
        app.add_systems(Update, check_textures.run_if(in_state(AppState::Setup)));
        app.add_systems(Update, keyboard_input_system.run_if(in_state(AppState::Finished)));
        app.add_systems(OnEnter(AppState::Finished), setup);

    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
  #[default]
  Setup,
  Finished,
}

#[derive(Resource, Default)]
struct KeyboardSpriteFolder(Handle<LoadedFolder>);

fn load_textures(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut keyBoards: ResMut<DisplayKeys>,
) {
  let boardkeys: Vec<ViewKey> = vec![
    ViewKey{press_id:96,release_id:97,key:KeyCode::Escape,row:0}, // esc
    ViewKey{press_id:102,release_id:109,key:KeyCode::F1,row:0}, // F1
    ViewKey{press_id:110,release_id:111,key:KeyCode::F2,row:0}, // F2
    ViewKey{press_id:112,release_id:113,key:KeyCode::F3,row:0}, // F3
    ViewKey{press_id:114,release_id:115,key:KeyCode::F4,row:0}, // F4
    ViewKey{press_id:116,release_id:117,key:KeyCode::F5,row:0}, // F5
    ViewKey{press_id:118,release_id:119,key:KeyCode::F6,row:0}, // F6
    ViewKey{press_id:120,release_id:121,key:KeyCode::F7,row:0}, // F7
    ViewKey{press_id:122,release_id:123,key:KeyCode::F8,row:0}, // F8
    ViewKey{press_id:124,release_id:125,key:KeyCode::F9,row:0}, // F9
    ViewKey{press_id:103,release_id:104,key:KeyCode::F10,row:0}, // F10
    ViewKey{press_id:105,release_id:106,key:KeyCode::F11,row:0}, // F11
    ViewKey{press_id:107,release_id:108,key:KeyCode::F12,row:0}, // F12

    ViewKey{press_id:206,release_id:207,key:KeyCode::Backquote,row:1}, // ~
    ViewKey{press_id:2,release_id:3,key:KeyCode::Digit1,row:1}, // 1
    ViewKey{press_id:4,release_id:5,key:KeyCode::Digit2,row:1}, // 2
    ViewKey{press_id:6,release_id:7,key:KeyCode::Digit3,row:1}, // 3
    ViewKey{press_id:8,release_id:9,key:KeyCode::Digit4,row:1}, // 4
    ViewKey{press_id:10,release_id:11,key:KeyCode::Digit5,row:1},
    ViewKey{press_id:12,release_id:13,key:KeyCode::Digit6,row:1},
    ViewKey{press_id:14,release_id:15,key:KeyCode::Digit7,row:1},
    ViewKey{press_id:16,release_id:17,key:KeyCode::Digit8,row:1},
    ViewKey{press_id:18,release_id:19,key:KeyCode::Digit9,row:1},
    ViewKey{press_id:0,release_id:1,key:KeyCode::Digit0,row:1},// 0

    ViewKey{press_id:146,release_id:147,key:KeyCode::Minus,row:1},
    ViewKey{press_id:94,release_id:95,key:KeyCode::Equal,row:1},
    ViewKey{press_id:55,release_id:58,key:KeyCode::Backspace,row:1},

    ViewKey{press_id:201,release_id:204,key:KeyCode::Tab,row:2}, // Tab
    ViewKey{press_id:173,release_id:178,key:KeyCode::KeyQ,row:2}, // Q
    ViewKey{press_id:213,release_id:216,key:KeyCode::KeyW,row:2}, // W
    ViewKey{press_id:89,release_id:100,key:KeyCode::KeyE,row:2}, // E
    ViewKey{press_id:179,release_id:182,key:KeyCode::KeyR,row:2}, // R
    ViewKey{press_id:199,release_id:208,key:KeyCode::KeyT,row:2}, // T
    ViewKey{press_id:219,release_id:220,key:KeyCode::KeyY,row:2}, // Y
    ViewKey{press_id:209,release_id:210,key:KeyCode::KeyU,row:2}, // U
    ViewKey{press_id:135,release_id:138,key:KeyCode::KeyI,row:2}, // I
    ViewKey{press_id:157,release_id:160,key:KeyCode::KeyO,row:2}, // O
    ViewKey{press_id:161,release_id:172,key:KeyCode::KeyP,row:2}, // P
    ViewKey{press_id:66,release_id:67,key:KeyCode::BracketLeft,row:2}, // [
    ViewKey{press_id:60,release_id:61,key:KeyCode::BracketRight,row:2}, // ]
    ViewKey{press_id:190,release_id:191,key:KeyCode::Backslash,row:2}, // \

    ViewKey{press_id:70,release_id:73,key:KeyCode::CapsLock,row:3}, // cap lock
    ViewKey{press_id:20,release_id:52,key:KeyCode::KeyA,row:3}, // A
    ViewKey{press_id:183,release_id:198,key:KeyCode::KeyS,row:3}, // S
    ViewKey{press_id:85,release_id:88,key:KeyCode::KeyD,row:3}, // D
    ViewKey{press_id:101,release_id:128,key:KeyCode::KeyF,row:3}, // F
    ViewKey{press_id:129,release_id:130,key:KeyCode::KeyG,row:3}, // G
    ViewKey{press_id:131,release_id:134,key:KeyCode::KeyH,row:3}, // H
    ViewKey{press_id:139,release_id:140,key:KeyCode::KeyJ,row:3}, // J
    ViewKey{press_id:141,release_id:142,key:KeyCode::KeyK,row:3}, // K
    ViewKey{press_id:143,release_id:144,key:KeyCode::KeyL,row:3}, // L
    ViewKey{press_id:184,release_id:185,key:KeyCode::Semicolon,row:3}, // ;
    ViewKey{press_id:25,release_id:26,key:KeyCode::Quote,row:3}, // '
    ViewKey{press_id:92,release_id:93,key:KeyCode::Enter,row:3}, // Enter

    ViewKey{press_id:187,release_id:188,key:KeyCode::ShiftLeft,row:4}, // shift
    ViewKey{press_id:221,release_id:222,key:KeyCode::KeyZ,row:4}, // Z
    ViewKey{press_id:217,release_id:218,key:KeyCode::KeyX,row:4}, // X
    ViewKey{press_id:69,release_id:84,key:KeyCode::KeyC,row:4}, // C
    ViewKey{press_id:211,release_id:212,key:KeyCode::KeyV,row:4}, // V
    ViewKey{press_id:53,release_id:68,key:KeyCode::KeyB,row:4}, // B
    ViewKey{press_id:149,release_id:156,key:KeyCode::KeyN,row:4}, // N
    ViewKey{press_id:145,release_id:148,key:KeyCode::KeyM,row:4}, // M
    ViewKey{press_id:78,release_id:81,key:KeyCode::Comma,row:4}, // ,
    ViewKey{press_id:166,release_id:167,key:KeyCode::Period,row:4}, // .
    ViewKey{press_id:192,release_id:193,key:KeyCode::Slash,row:4}, // /
    ViewKey{press_id:187,release_id:188,key:KeyCode::ShiftRight,row:4}, // Shift

    ViewKey{press_id:82,release_id:83,key:KeyCode::ControlLeft,row:5}, // Ctrl left
    ViewKey{press_id:214,release_id:215,key:KeyCode::SuperLeft,row:5}, // window
    ViewKey{press_id:21,release_id:22,key:KeyCode::AltLeft,row:5}, // Alt Left
    ViewKey{press_id:195,release_id:196,key:KeyCode::Space,row:5}, // Space bar
    ViewKey{press_id:21,release_id:22,key:KeyCode::AltRight,row:5}, // Alt Right
    ViewKey{press_id:214,release_id:215,key:KeyCode::SuperRight,row:5}, // Window
    ViewKey{press_id:126,release_id:127,key:KeyCode::Fn,row:5}, // FN
    ViewKey{press_id:82,release_id:83,key:KeyCode::ControlRight,row:5}, // Ctrl Right


    //ViewKey{press_id:82,release_id:51,key:KeyCode::ControlRight,row:6}, //
    //ViewKey{press_id:82,release_id:52,key:KeyCode::ControlRight,row:6}, //
    //ViewKey{press_id:82,release_id:53,key:KeyCode::ControlRight,row:6}, //
    //ViewKey{press_id:82,release_id:54,key:KeyCode::ControlRight,row:6}, //




    //ViewKey{press_id:0,release_id:19,key:KeyCode::ControlRight,row:6}, // 9
    //ViewKey{press_id:0,release_id:20,key:KeyCode::Numpad9,row:2}, // A white
    //ViewKey{press_id:0,release_id:21,key:KeyCode::Numpad9,row:2}, // Alt white
    //ViewKey{press_id:0,release_id:22,key:KeyCode::Numpad9,row:2}, // alt
    //ViewKey{press_id:0,release_id:23,key:KeyCode::Numpad9,row:2}, // ANY white
    //ViewKey{press_id:0,release_id:24,key:KeyCode::Numpad9,row:2}, // any 
    //ViewKey{press_id:0,release_id:25,key:KeyCode::Numpad9,row:2}, // ' white
    //ViewKey{press_id:0,release_id:26,key:KeyCode::Numpad9,row:2}, // '
    //ViewKey{press_id:0,release_id:27,key:KeyCode::Numpad9,row:2}, // arrow keys white
    //ViewKey{press_id:0,release_id:28,key:KeyCode::Numpad9,row:2}, // arrow key red
    //ViewKey{press_id:0,release_id:29,key:KeyCode::Numpad9,row:2}, // arrow key red mid
    //ViewKey{press_id:0,release_id:30,key:KeyCode::Numpad9,row:2}, //  arrow key

    //ViewKey{press_id:0,release_id:31,key:KeyCode::Numpad9,row:2}, //  arrow key
    //ViewKey{press_id:0,release_id:32,key:KeyCode::Numpad9,row:2}, //  arrow key
    //ViewKey{press_id:0,release_id:33,key:KeyCode::Numpad9,row:2}, // arrow key
    //ViewKey{press_id:0,release_id:34,key:KeyCode::Numpad9,row:2}, //  arrow key
    //ViewKey{press_id:0,release_id:35,key:KeyCode::Numpad9,row:2}, //  arrow key
    //ViewKey{press_id:0,release_id:36,key:KeyCode::Numpad9,row:2}, //  arrow key
    //ViewKey{press_id:0,release_id:37,key:KeyCode::Numpad9,row:2}, //  arrow key
    //ViewKey{press_id:0,release_id:38,key:KeyCode::Numpad9,row:2}, //  arrow key
    //ViewKey{press_id:0,release_id:39,key:KeyCode::Numpad9,row:2}, //  arrow key
    //ViewKey{press_id:0,release_id:40,key:KeyCode::Numpad9,row:2}, //  arrow key

    //ViewKey{press_id:0,release_id:41,key:KeyCode::Numpad9,row:2}, // arrow key
    //ViewKey{press_id:0,release_id:42,key:KeyCode::Numpad9,row:2}, // down white
    //ViewKey{press_id:0,release_id:43,key:KeyCode::Numpad9,row:2}, // down
    //ViewKey{press_id:0,release_id:45,key:KeyCode::Numpad9,row:2}, // left 
    //ViewKey{press_id:0,release_id:46,key:KeyCode::Numpad9,row:2}, // right white
    //ViewKey{press_id:0,release_id:47,key:KeyCode::Numpad9,row:2}, // right
    //ViewKey{press_id:0,release_id:48,key:KeyCode::Numpad9,row:2}, // up white
    //ViewKey{press_id:0,release_id:49,key:KeyCode::Numpad9,row:2}, // up
    //ViewKey{press_id:0,release_id:50,key:KeyCode::Numpad9,row:2}, // * white

    //ViewKey{press_id:0,release_id:51,key:KeyCode::Numpad9,row:2}, // *
    //ViewKey{press_id:0,release_id:52,key:KeyCode::Numpad9,row:2}, // A
    //ViewKey{press_id:0,release_id:53,key:KeyCode::Numpad9,row:2}, // B White
    //ViewKey{press_id:0,release_id:54,key:KeyCode::Numpad9,row:2}, // back space text
    //ViewKey{press_id:0,release_id:55,key:KeyCode::Numpad9,row:2}, // back space white arrow
    //ViewKey{press_id:0,release_id:56,key:KeyCode::Numpad9,row:2}, // back space icon
    //ViewKey{press_id:0,release_id:57,key:KeyCode::Numpad9,row:2}, // back space icon
    //ViewKey{press_id:0,release_id:58,key:KeyCode::Numpad9,row:2}, // back space arrow
    //ViewKey{press_id:0,release_id:59,key:KeyCode::Numpad9,row:2}, // back space text

    //ViewKey{press_id:0,release_id:60,key:KeyCode::Numpad9,row:2}, // ] white
    //ViewKey{press_id:0,release_id:61,key:KeyCode::Numpad9,row:2}, // ]
    //ViewKey{press_id:0,release_id:62,key:KeyCode::Numpad9,row:2}, // > white
    //ViewKey{press_id:0,release_id:63,key:KeyCode::Numpad9,row:2}, // >
    //ViewKey{press_id:0,release_id:64,key:KeyCode::Numpad9,row:2}, // < white
    //ViewKey{press_id:0,release_id:65,key:KeyCode::Numpad9,row:2}, // <
    //ViewKey{press_id:0,release_id:66,key:KeyCode::Numpad9,row:2}, // [ white
    //ViewKey{press_id:0,release_id:67,key:KeyCode::Numpad9,row:2}, // [
    //ViewKey{press_id:0,release_id:68,key:KeyCode::Numpad9,row:2}, // B
    //ViewKey{press_id:0,release_id:69,key:KeyCode::Numpad9,row:2}, // C White

    //ViewKey{press_id:0,release_id:70,key:KeyCode::Numpad9,row:2}, // cap lock white
    //ViewKey{press_id:0,release_id:71,key:KeyCode::Numpad9,row:2}, // shift white text
    //ViewKey{press_id:0,release_id:72,key:KeyCode::Numpad9,row:2}, // shift text
    //ViewKey{press_id:0,release_id:73,key:KeyCode::Numpad9,row:2}, // cap lock
    //ViewKey{press_id:0,release_id:74,key:KeyCode::Numpad9,row:2}, // ^ white
    //ViewKey{press_id:0,release_id:75,key:KeyCode::Numpad9,row:2}, // ^
    //ViewKey{press_id:0,release_id:76,key:KeyCode::Numpad9,row:2}, // : white
    //ViewKey{press_id:0,release_id:78,key:KeyCode::Numpad9,row:2}, // , white
    //ViewKey{press_id:0,release_id:79,key:KeyCode::Numpad9,row:2}, // comand white
    //ViewKey{press_id:0,release_id:80,key:KeyCode::Numpad9,row:2}, // comand

    //ViewKey{press_id:0,release_id:81,key:KeyCode::Numpad9,row:2}, // ,
    //ViewKey{press_id:0,release_id:82,key:KeyCode::Numpad9,row:2}, // Ctrl white
    //ViewKey{press_id:0,release_id:83,key:KeyCode::Numpad9,row:2}, // Ctrl
    //ViewKey{press_id:0,release_id:84,key:KeyCode::Numpad9,row:2}, // C 
    //ViewKey{press_id:0,release_id:85,key:KeyCode::Numpad9,row:2}, // D white
    //ViewKey{press_id:0,release_id:86,key:KeyCode::Numpad9,row:2}, // Del white
    //ViewKey{press_id:0,release_id:87,key:KeyCode::Numpad9,row:2}, // Del
    //ViewKey{press_id:0,release_id:88,key:KeyCode::Numpad9,row:2}, // D
    //ViewKey{press_id:0,release_id:89,key:KeyCode::Numpad9,row:2}, // E white
    //ViewKey{press_id:0,release_id:90,key:KeyCode::Numpad9,row:2}, // End white


    //ViewKey{press_id:0,release_id:91,key:KeyCode::Numpad9,row:2}, // End
    //ViewKey{press_id:0,release_id:92,key:KeyCode::Numpad9,row:2}, // Enter white
    //ViewKey{press_id:0,release_id:93,key:KeyCode::Numpad9,row:2}, // Enter
    //ViewKey{press_id:0,release_id:94,key:KeyCode::Numpad9,row:2}, // = white
    //ViewKey{press_id:0,release_id:95,key:KeyCode::Numpad9,row:2}, // =
    //ViewKey{press_id:0,release_id:96,key:KeyCode::Numpad9,row:2}, // Esc white
    //ViewKey{press_id:0,release_id:97,key:KeyCode::Numpad9,row:2}, // esc
    //ViewKey{press_id:0,release_id:98,key:KeyCode::Numpad9,row:2}, // !
    //ViewKey{press_id:0,release_id:99,key:KeyCode::Numpad9,row:2}, // !
    //ViewKey{press_id:0,release_id:100,key:KeyCode::Numpad9,row:2}, // E

    //ViewKey{press_id:0,release_id:101,key:KeyCode::Numpad9,row:2},// F white
    //ViewKey{press_id:0,release_id:102,key:KeyCode::Numpad9,row:2},// F1 white
    //ViewKey{press_id:0,release_id:103,key:KeyCode::Numpad9,row:2},// F10 white
    //ViewKey{press_id:0,release_id:104,key:KeyCode::Numpad9,row:2},// F10
    //ViewKey{press_id:0,release_id:105,key:KeyCode::Numpad9,row:2},// F11 white
    //ViewKey{press_id:0,release_id:106,key:KeyCode::Numpad9,row:2},// F11 
    //ViewKey{press_id:0,release_id:107,key:KeyCode::Numpad9,row:2},// F12 white
    //ViewKey{press_id:0,release_id:108,key:KeyCode::Numpad9,row:2},// F12
    //ViewKey{press_id:0,release_id:109,key:KeyCode::Numpad9,row:2},// F1
    //ViewKey{press_id:0,release_id:110,key:KeyCode::Numpad9,row:2},// F2 white 

    //ViewKey{press_id:0,release_id:111,key:KeyCode::Numpad9,row:2},// F2
    //ViewKey{press_id:0,release_id:112,key:KeyCode::Numpad9,row:2},// F3 white
    //ViewKey{press_id:0,release_id:113,key:KeyCode::Numpad9,row:2},// F3
    //ViewKey{press_id:0,release_id:114,key:KeyCode::Numpad9,row:2},// F4 white
    //ViewKey{press_id:0,release_id:115,key:KeyCode::Numpad9,row:2},// F4
    //ViewKey{press_id:0,release_id:116,key:KeyCode::Numpad9,row:2},// F5 white
    //ViewKey{press_id:0,release_id:117,key:KeyCode::Numpad9,row:2},// F5
    //ViewKey{press_id:0,release_id:118,key:KeyCode::Numpad9,row:2},// F6 white
    //ViewKey{press_id:0,release_id:119,key:KeyCode::Numpad9,row:2},// F6
    //ViewKey{press_id:0,release_id:120,key:KeyCode::Numpad9,row:2},// F7 white


    // ViewKey{press_id:0,release_id:121,key:KeyCode::Numpad9,row:2},// F7
    // ViewKey{press_id:0,release_id:122,key:KeyCode::Numpad9,row:2},// F8 whjite
    // ViewKey{press_id:0,release_id:123,key:KeyCode::Numpad9,row:2},// F8
    // ViewKey{press_id:0,release_id:124,key:KeyCode::Numpad9,row:2},// F9 white
    // ViewKey{press_id:0,release_id:125,key:KeyCode::Numpad9,row:2},// F9
    // ViewKey{press_id:0,release_id:126,key:KeyCode::Numpad9,row:2},// FN white
    // ViewKey{press_id:0,release_id:127,key:KeyCode::Numpad9,row:2},// FN
    // ViewKey{press_id:0,release_id:128,key:KeyCode::Numpad9,row:2},// F
    // ViewKey{press_id:0,release_id:129,key:KeyCode::Numpad9,row:2},// G white
    // ViewKey{press_id:0,release_id:130,key:KeyCode::Numpad9,row:2},// G

    // ViewKey{press_id:0,release_id:131,key:KeyCode::Numpad9,row:2},// H white
    // ViewKey{press_id:0,release_id:132,key:KeyCode::Numpad9,row:2},// Home white
    // ViewKey{press_id:0,release_id:133,key:KeyCode::Numpad9,row:2},// Home
    // ViewKey{press_id:0,release_id:134,key:KeyCode::Numpad9,row:2},// H
    // ViewKey{press_id:0,release_id:135,key:KeyCode::Numpad9,row:2},// I white
    // ViewKey{press_id:0,release_id:136,key:KeyCode::Numpad9,row:2},// INS white
    // ViewKey{press_id:0,release_id:137,key:KeyCode::Numpad9,row:2},// INS
    // ViewKey{press_id:0,release_id:138,key:KeyCode::Numpad9,row:2},// I
    // ViewKey{press_id:0,release_id:139,key:KeyCode::Numpad9,row:2},// J white
    // ViewKey{press_id:0,release_id:140,key:KeyCode::Numpad9,row:2},// J

    // ViewKey{press_id:0,release_id:141,key:KeyCode::Numpad9,row:2},// K white
    // ViewKey{press_id:0,release_id:142,key:KeyCode::Numpad9,row:2},// K
    // ViewKey{press_id:0,release_id:143,key:KeyCode::Numpad9,row:2},// L white
    // ViewKey{press_id:0,release_id:144,key:KeyCode::Numpad9,row:2},// L
    // ViewKey{press_id:0,release_id:145,key:KeyCode::Numpad9,row:2},// m white
    // ViewKey{press_id:0,release_id:146,key:KeyCode::Numpad9,row:2},// - white
    // ViewKey{press_id:0,release_id:147,key:KeyCode::Numpad9,row:2},// - 
    // ViewKey{press_id:0,release_id:148,key:KeyCode::Numpad9,row:2},// m
    // ViewKey{press_id:0,release_id:149,key:KeyCode::Numpad9,row:2},// n white
    // ViewKey{press_id:0,release_id:150,key:KeyCode::Numpad9,row:2},// num lock white

    // ViewKey{press_id:0,release_id:151,key:KeyCode::Numpad9,row:2},// num lock
    // ViewKey{press_id:0,release_id:152,key:KeyCode::Numpad9,row:2},// enter white
    // ViewKey{press_id:0,release_id:153,key:KeyCode::Numpad9,row:2},// enter
    // ViewKey{press_id:0,release_id:154,key:KeyCode::Numpad9,row:2},// + white
    // ViewKey{press_id:0,release_id:155,key:KeyCode::Numpad9,row:2},// +
    // ViewKey{press_id:0,release_id:156,key:KeyCode::Numpad9,row:2},// n
    // ViewKey{press_id:0,release_id:157,key:KeyCode::Numpad9,row:2},// o white
    // ViewKey{press_id:0,release_id:158,key:KeyCode::Numpad9,row:2},// option white
    // ViewKey{press_id:0,release_id:159,key:KeyCode::Numpad9,row:2},// option 
    // ViewKey{press_id:0,release_id:160,key:KeyCode::Numpad9,row:2},// o

    // ViewKey{press_id:0,release_id:161,key:KeyCode::Numpad9,row:2}, // p white
    // ViewKey{press_id:0,release_id:162,key:KeyCode::Numpad9,row:2}, // page down white
    // ViewKey{press_id:0,release_id:163,key:KeyCode::Numpad9,row:2}, // page down 
    // ViewKey{press_id:0,release_id:164,key:KeyCode::Numpad9,row:2}, // page up white
    // ViewKey{press_id:0,release_id:165,key:KeyCode::Numpad9,row:2}, // page up
    // ViewKey{press_id:0,release_id:166,key:KeyCode::Numpad9,row:2}, // . white
    // ViewKey{press_id:0,release_id:167,key:KeyCode::Numpad9,row:2}, // .
    // ViewKey{press_id:0,release_id:168,key:KeyCode::Numpad9,row:2}, // + white
    // ViewKey{press_id:0,release_id:169,key:KeyCode::Numpad9,row:2}, // +
    // ViewKey{press_id:0,release_id:170,key:KeyCode::Numpad9,row:2}, // prt scrn white


    // ViewKey{press_id:0,release_id:171,key:KeyCode::Numpad9,row:2}, // prt scrn
    // ViewKey{press_id:0,release_id:172,key:KeyCode::Numpad9,row:2}, // P 
    // ViewKey{press_id:0,release_id:173,key:KeyCode::Numpad9,row:2}, // Q white
    // ViewKey{press_id:0,release_id:174,key:KeyCode::Numpad9,row:2}, // ? white
    // ViewKey{press_id:0,release_id:175,key:KeyCode::Numpad9,row:2}, // ?
    // ViewKey{press_id:0,release_id:176,key:KeyCode::Numpad9,row:2}, // " white
    // ViewKey{press_id:0,release_id:177,key:KeyCode::Numpad9,row:2}, // "
    // ViewKey{press_id:0,release_id:178,key:KeyCode::Numpad9,row:2}, // Q
    // ViewKey{press_id:0,release_id:179,key:KeyCode::Numpad9,row:2}, // R white
    // ViewKey{press_id:0,release_id:180,key:KeyCode::Numpad9,row:2}, // enter icon white
  
    // ViewKey{press_id:0,release_id:181,key:KeyCode::Numpad9,row:2}, // enter icon
    // ViewKey{press_id:0,release_id:182,key:KeyCode::Numpad9,row:2}, // R
    // ViewKey{press_id:0,release_id:183,key:KeyCode::Numpad9,row:2}, // S white
    // ViewKey{press_id:0,release_id:184,key:KeyCode::Numpad9,row:2}, // ; white
    // ViewKey{press_id:0,release_id:185,key:KeyCode::Numpad9,row:2}, // ;
    // ViewKey{press_id:0,release_id:186,key:KeyCode::Numpad9,row:2}, // shift white text 
    // ViewKey{press_id:0,release_id:187,key:KeyCode::Numpad9,row:2}, // shift up arrow white
    // ViewKey{press_id:0,release_id:188,key:KeyCode::Numpad9,row:2}, // shift up arrow
    // ViewKey{press_id:0,release_id:189,key:KeyCode::Numpad9,row:2}, // shift text
    // ViewKey{press_id:0,release_id:190,key:KeyCode::Numpad9,row:2}, // \ white

    // ViewKey{press_id:0,release_id:191,key:KeyCode::Numpad9,row:2}, // \
    // ViewKey{press_id:0,release_id:192,key:KeyCode::Numpad9,row:2}, // / white
    // ViewKey{press_id:0,release_id:193,key:KeyCode::Numpad9,row:2}, // /
    // ViewKey{press_id:0,release_id:194,key:KeyCode::Numpad9,row:2}, // spacebar text white 
    // ViewKey{press_id:0,release_id:195,key:KeyCode::Numpad9,row:2}, // spacebar icon white
    // ViewKey{press_id:0,release_id:196,key:KeyCode::Numpad9,row:2}, // spacebar icon
    // ViewKey{press_id:0,release_id:197,key:KeyCode::Numpad9,row:2}, // spacebar text
    // ViewKey{press_id:0,release_id:198,key:KeyCode::Numpad9,row:2}, // S
    // ViewKey{press_id:0,release_id:199,key:KeyCode::Numpad9,row:2}, // T white
    // ViewKey{press_id:0,release_id:200,key:KeyCode::Numpad9,row:2}, // tab text white

    // ViewKey{press_id:0,release_id:201,key:KeyCode::Numpad9,row:2}, // tab 2 arrow white
    // ViewKey{press_id:0,release_id:202,key:KeyCode::Numpad9,row:2}, // tab 2 arrow white
    // ViewKey{press_id:0,release_id:203,key:KeyCode::Numpad9,row:2}, // tab arrow
    // ViewKey{press_id:0,release_id:204,key:KeyCode::Numpad9,row:2}, // tab 2 arrow
    // ViewKey{press_id:0,release_id:205,key:KeyCode::Numpad9,row:2}, // tab text
    // ViewKey{press_id:0,release_id:206,key:KeyCode::Numpad9,row:2}, // Backquote white
    // ViewKey{press_id:0,release_id:207,key:KeyCode::Numpad9,row:2}, // Backquote
    // ViewKey{press_id:0,release_id:208,key:KeyCode::Numpad9,row:2}, // T
    // ViewKey{press_id:0,release_id:209,key:KeyCode::Numpad9,row:2}, // U white
    // ViewKey{press_id:0,release_id:210,key:KeyCode::Numpad9,row:2}, // U

    // ViewKey{press_id:0,release_id:211,key:KeyCode::Numpad9,row:2}, // V white
    // ViewKey{press_id:0,release_id:212,key:KeyCode::Numpad9,row:2}, // V
    // ViewKey{press_id:0,release_id:213,key:KeyCode::Numpad9,row:2}, // w white
    // ViewKey{press_id:0,release_id:214,key:KeyCode::Numpad9,row:2}, // window icon white
    // ViewKey{press_id:0,release_id:215,key:KeyCode::Numpad9,row:2}, // window icon
    // ViewKey{press_id:0,release_id:216,key:KeyCode::Numpad9,row:2}, // w
    // ViewKey{press_id:0,release_id:217,key:KeyCode::Numpad9,row:2}, // x white
    // ViewKey{press_id:0,release_id:218,key:KeyCode::Numpad9,row:2}, // x
    // ViewKey{press_id:0,release_id:219,key:KeyCode::Numpad9,row:2}, // y white
    // ViewKey{press_id:0,release_id:220,key:KeyCode::Numpad9,row:2}, // y

    //ViewKey{press_id:0,release_id:221,key:KeyCode::Numpad9,row:2}, // z white
    //ViewKey{press_id:0,release_id:222,key:KeyCode::Numpad9,row:2}, // z
    //ViewKey{press_id:0,release_id:223,key:KeyCode::Numpad9,row:2}, // mouse white
    //ViewKey{press_id:0,release_id:224,key:KeyCode::Numpad9,row:2}, // mouse horizontal
    //ViewKey{press_id:0,release_id:225,key:KeyCode::Numpad9,row:2}, // mouse left white
    //ViewKey{press_id:0,release_id:226,key:KeyCode::Numpad9,row:2}, // mouse left
    //ViewKey{press_id:0,release_id:227,key:KeyCode::Numpad9,row:2}, // mouse move white
    //ViewKey{press_id:0,release_id:228,key:KeyCode::Numpad9,row:2}, // mouse 
    //ViewKey{press_id:0,release_id:229,key:KeyCode::Numpad9,row:2}, // mouse right white
    //ViewKey{press_id:0,release_id:230,key:KeyCode::Numpad9,row:2}, // mouse right

    //ViewKey{press_id:0,release_id:231,key:KeyCode::Numpad9,row:2}, // mouse middle white
    //ViewKey{press_id:0,release_id:232,key:KeyCode::Numpad9,row:2}, // mouse middle white
    //ViewKey{press_id:0,release_id:233,key:KeyCode::Numpad9,row:2}, // mouse wheel down
    //ViewKey{press_id:0,release_id:234,key:KeyCode::Numpad9,row:2}, // mouse middle
    //ViewKey{press_id:0,release_id:235,key:KeyCode::Numpad9,row:2}, // mouse middle up red white
    //ViewKey{press_id:0,release_id:236,key:KeyCode::Numpad9,row:2}, // mouse middle up
    //ViewKey{press_id:0,release_id:237,key:KeyCode::Numpad9,row:2}, // mouse up red white
    //ViewKey{press_id:0,release_id:238,key:KeyCode::Numpad9,row:2}, // mouse vertial
    //ViewKey{press_id:0,release_id:239,key:KeyCode::Numpad9,row:2}, // mouse white
    //ViewKey{press_id:0,release_id:240,key:KeyCode::Numpad9,row:2}, // mouse vertial white
    //ViewKey{press_id:0,release_id:241,key:KeyCode::Numpad9,row:2}, // 

    // 241 icons over limited texture atlas
    
    




  ];


  keyBoards.key_list = boardkeys;
  // load multiple, individual sprites from a folder
  //commands.insert_resource(KeyboardSpriteFolder(asset_server.load_folder("kenney_input-prompts/Keyboard & Mouse/Default")));
  commands.insert_resource(KeyboardSpriteFolder(asset_server.load_folder("kenney_input-prompts/Keyboard & Mouse")));
}

fn check_textures(
  mut next_state: ResMut<NextState<AppState>>,
  rpg_sprite_folder: Res<KeyboardSpriteFolder>,
  mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
  // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
  for event in events.read() {
      if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
          next_state.set(AppState::Finished);
      }
  }
}

fn setup(
  mut commands: Commands,
  rpg_sprite_handles: Res<KeyboardSpriteFolder>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
  loaded_folders: Res<Assets<LoadedFolder>>,
  mut textures: ResMut<Assets<Image>>,
  mut displaykeys: ResMut<DisplayKeys>,
) {
  let loaded_folder = loaded_folders.get(&rpg_sprite_handles.0).unwrap();

  // create texture atlases with different padding and sampling

  let (texture_atlas_linear, linear_texture) = create_texture_atlas(
      loaded_folder,
      None,
      Some(ImageSampler::linear()),
      &mut textures,
  );
  let atlas_linear_handle = texture_atlases.add(texture_atlas_linear.clone());

  // setup 2d scene
  commands.spawn(Camera2dBundle::default());

  // padded textures are to the right, unpadded to the left

  // draw unpadded texture atlas
  commands.spawn(SpriteBundle {
      texture: linear_texture.clone(),
      transform: Transform {
          translation: Vec3::new(-200.0, -20.0, 0.0),
          scale: Vec3::splat(0.6),
          //scale: Vec3::splat(1.0),
          ..default()
      },
      ..default()
  });
  

  let font = asset_server.load("fonts/FiraSans-Bold.ttf");

  // padding label text style
  let text_style: TextStyle = TextStyle {
      font: font.clone(),
      font_size: 50.0,
      color: Color::WHITE,
  };

  // labels to indicate padding

  // No padding
  // create_label(
  //     &mut commands,
  //     (-250.0, 330.0, 0.0),
  //     "No padding",
  //     text_style.clone(),
  // );

  // Padding
  //create_label(&mut commands, (250.0, 330.0, 0.0), "Padding", text_style);

  let enity_keyboard = commands
  .spawn(NodeBundle {
          style: Style {
              align_items: AlignItems::Center,
              justify_content: JustifyContent::Center,
              flex_direction: FlexDirection::Column,
              //width: Val::Percent(100.0),
              //height: Val::Percent(100.0),
              width: Val::Px(900.0),
              height: Val::Px(384.0),
              ..default()
          },
          //background_color: Color::CRIMSON.into(),
          background_color: Color::BLUE.into(),
          ..default()
      }).id();

  let enity_keyboard_row1 = commands
    .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                //flex_direction: FlexDirection::Row,
                //width: Val::Percent(100.0),
                //height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::CRIMSON.into(),
            ..default()
        }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row1]);

  //let mut base_y = 128.0; // y position of the sprites
  let mut base_y = 0.0; // y position of the sprites
  let mut x = -300.;
  let mut row = 0;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      let entity_button = create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key,
        &mut texture_atlases
      );
      //commands.entity(enity_keyboard).add_child(entity_button);
      commands.entity(enity_keyboard_row1).push_children(&[entity_button]);
    x += 32.;
    }
  }
  
  let enity_keyboard_row2 = commands
    .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                ..default()
            },
            ..default()
        }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row2]);

  x = -300.;
  base_y = 96.;
  row = 1;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      let entity_button = create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key,
        &mut texture_atlases
      );
      x += 32.;
      commands.entity(enity_keyboard_row2).push_children(&[entity_button]);
    }
  }

  let enity_keyboard_row3 = commands
    .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                ..default()
            },
            ..default()
        }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row3]);
  
  x = -300.;
  base_y = 64.;
  row = 2;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      let entity_button = create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key,
        &mut texture_atlases
      );
      commands.entity(enity_keyboard_row3).push_children(&[entity_button]);
      x += 32.;
    }
  }

  let enity_keyboard_row4 = commands
    .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                ..default()
            },
            ..default()
        }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row4]);

  x = -300.;
  base_y = 32.;
  row = 3;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      let entity_button = create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key,
        &mut texture_atlases
      );
      commands.entity(enity_keyboard_row4).push_children(&[entity_button]);
      x += 32.;
    }
  }


  let enity_keyboard_row5 = commands
    .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                ..default()
            },
            ..default()
        }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row5]);

  x = -300.;
  base_y = 0.;
  row = 4;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      let entity_button = create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key,
        &mut texture_atlases
      );
      commands.entity(enity_keyboard_row5).push_children(&[entity_button]);
      x += 32.;
    }
  }


  let enity_keyboard_row6 = commands
    .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                ..default()
            },
            ..default()
        }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row6]);

  x = -300.;
  base_y = -32.;
  row = 5;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      let entity_button =create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key,
        &mut texture_atlases
      );
      commands.entity(enity_keyboard_row6).push_children(&[entity_button]);
      x += 32.;
    }
  }

  x = -300.;
  base_y = -64.;
  row = 6;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key,
        &mut texture_atlases
      );
      x += 32.;
    }
  }

  /*
  create_sprite_from_atlas(
    &mut commands,
    (x, base_y, 0.0),
    0,
    //atlas_handle,
    atlas_linear_handle,
    linear_texture,
  );
  */
}

fn keyboard_input_system(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut texture_atlas_query: Query<(&TagInputKey, &mut TextureAtlas)>,
  mut displaykeys: ResMut<DisplayKeys>,
){
  let shift = keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
  let ctrl = keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
  //look which key is press or release

  for mykey in  displaykeys.key_list.iter(){
    if keyboard_input.just_pressed(mykey.key) {

      for (tag_input_key, mut text_atlas ) in texture_atlas_query.iter_mut(){
        if tag_input_key.key == mykey.key {
          text_atlas.index = mykey.press_id;
          break;
        }
      }
    }
    if keyboard_input.just_released(mykey.key) {
      for (tag_input_key, mut text_atlas ) in texture_atlas_query.iter_mut(){
        if tag_input_key.key == mykey.key {
          text_atlas.index = mykey.release_id;
          break;
        }
      }
    }
  }

  /*
  if keyboard_input.just_pressed(KeyCode::Numpad0) {
    if let Ok(mut texAtlas) = texture_atlas_query.get_single_mut() {
      println!("{:?}",displaykeys.key_list);
      texAtlas.index = 0;
    }
  }
  if keyboard_input.just_released(KeyCode::Numpad0) {
    if let Ok(mut texAtlas) = texture_atlas_query.get_single_mut() {
      texAtlas.index = 1;
    }
  }
  */
}

fn create_texture_atlas(
  folder: &LoadedFolder,
  padding: Option<UVec2>,
  sampling: Option<ImageSampler>,
  textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
  // Build a texture atlas using the individual sprites
  let mut texture_atlas_builder =
      TextureAtlasBuilder::default().padding(padding.unwrap_or_default());
  for handle in folder.handles.iter() {
      let id = handle.id().typed_unchecked::<Image>();
      let Some(texture) = textures.get(id) else {
          warn!(
              "{:?} did not resolve to an `Image` asset.",
              handle.path().unwrap()
          );
          continue;
      };

      texture_atlas_builder.add_texture(Some(id), texture);
  }

  let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
  let texture = textures.add(texture);

  // Update the sampling settings of the texture atlas
  let image = textures.get_mut(&texture).unwrap();
  image.sampler = sampling.unwrap_or_default();

  (texture_atlas_layout, texture)
}

/// Create and spawn a sprite from a texture atlas
fn create_sprite_from_atlas(
  commands: &mut Commands,
  translation: (f32, f32, f32),
  sprite_index: usize,
  atlas_handle: Handle<TextureAtlasLayout>,
  texture: Handle<Image>,
  key:KeyCode,
  texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> Entity {
  /*
  let sprite = commands.spawn(SpriteSheetBundle {
      transform: Transform {
          //translation: Vec3::new(translation.0, translation.1, translation.2),
          scale: Vec3::splat(0.64),
          ..default()
      },
      texture,
      atlas: TextureAtlas {
          index: sprite_index,
          layout: atlas_handle,
      },
      ..default()
  })
  //.id()
  //.insert(Style::default())
  .insert(NodeBundle {
    style: Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        //flex_direction: FlexDirection::Column,
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),

        width: Val::Px(16.0),
        height: Val::Px(16.0),
        ..default()
    },
    //background_color: Color::CRIMSON.into(),
    background_color: Color::BLUE.into(),
    ..default()
  })
  //.insert(CalculatedClip{Val:Size::new(30.0, 30.0)})
  //.insert(Node::default())
  //.insert(NodeBundle::default())
  .insert(TagInputKey{key})
  //.insert(KEY0)
  .id();
  
  let mykey = commands
    .spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            height:Val::Px(32.0),
            width:Val::Px(32.0),
            //padding: Val::UiRect(8.0),
            margin: UiRect {
              left: Val::Px(2.0),
              right:Val::Px(2.0),
              top: Val::Px(2.0),
              bottom: Val::Px(2.0),
            },
            padding: UiRect {
              left: Val::Px(2.0),
              right:Val::Px(2.0),
              top: Val::Px(2.0),
              bottom: Val::Px(2.0),
            },
            ..default()
        },
        background_color: Color::CRIMSON.into(),
        ..default()
    }).id();
    //sprite
    commands.entity(mykey).push_children(&[sprite]);

    mykey
  */

  let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 16, 16, None, None);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);


  let enity_keyicon = commands
  .spawn(NodeBundle {
    style: Style {
      height: Val::Px(32.0),
      width: Val::Px(32.0),
      align_items: AlignItems::Default,
      justify_content: JustifyContent::Default,
      margin: UiRect {
        left: Val::Px(2.0),
        right:Val::Px(2.0),
        top: Val::Px(2.0),
        bottom: Val::Px(2.0),
      },
      ..default()
    },
    background_color: Color::ALICE_BLUE.into(),
    ..default()
  })
  //.insert(UiImage::new(texture))//ok but display sheet
  .insert(AtlasImageBundle {
    style: Style {
        width: Val::Px(64.),
        height: Val::Px(64.),
        ..default()
    },
    //texture_atlas: texture_atlas_handle.into(),
    texture_atlas:TextureAtlas {
      index: sprite_index,
      layout: atlas_handle,
    },
    image: UiImage::new(texture),
    ..default()
  }).insert(TagInputKey{key})
  /*
  .insert(SpriteSheetBundle {
    transform: Transform {
        //translation: Vec3::new(translation.0, translation.1, translation.2),
        scale: Vec3::splat(0.64),
        ..default()
    },
    texture,
    atlas: TextureAtlas {
        index: sprite_index,
        layout: atlas_handle,
    },
    ..default()
  })*/
  .id();

  enity_keyicon

}


/*
fn create_sprite_from_atlas(
  commands: &mut Commands,
  translation: (f32, f32, f32),
  sprite_index: usize,
  atlas_handle: Handle<TextureAtlasLayout>,
  texture: Handle<Image>,
) {
  commands.spawn(SpriteSheetBundle {
      transform: Transform {
          translation: Vec3::new(translation.0, translation.1, translation.2),
          scale: Vec3::splat(3.0),
          ..default()
      },
      texture,
      atlas: TextureAtlas {
          index: sprite_index,
          layout: atlas_handle,
      },
      ..default()
  }).insert(KEY0);
}
*/

/// Create and spawn a label (text)
fn create_label(
  commands: &mut Commands,
  translation: (f32, f32, f32),
  text: &str,
  text_style: TextStyle,
) {
  commands.spawn(Text2dBundle {
      text: Text::from_section(text, text_style).with_justify(JustifyText::Center),
      transform: Transform {
          translation: Vec3::new(translation.0, translation.1, translation.2),
          ..default()
      },
      ..default()
  });
}