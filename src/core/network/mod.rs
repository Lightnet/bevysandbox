/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */
//use bevy::prelude::*;

use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
    },
    transport::{NetcodeClientPlugin, NetcodeServerPlugin},
    RenetClientPlugin, RenetServerPlugin,
};
use renet::{
  transport::{NetcodeClientTransport, NetcodeServerTransport, NetcodeTransportError},
  ClientId,
};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

use serde::{Deserialize, Serialize};

const PROTOCOL_ID: u64 = 7;

const PLAYER_MOVE_SPEED: f32 = 1.0;

#[derive(Debug, Default, Serialize, Deserialize, Component, Resource)]
struct PlayerInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Component)]
struct Player {
  id: ClientId,
}

#[derive(Debug, Default, Resource)]
struct Lobby {
  players: HashMap<ClientId, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
enum ServerMessages {
  PlayerConnected { id: u64 },
  PlayerDisconnected { id: u64 },
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(ConnectionConfig::default());

    (server, transport)
}


pub struct SandboxNetworkPlugin;

impl Plugin for SandboxNetworkPlugin {
  fn build(&self, app: &mut App) {
    //app
    //server
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);

    //client
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(NetcodeClientPlugin);

    //log
    app.add_systems(Update, panic_on_error_system);

  }
}

// If any error is found we just panic
#[allow(clippy::never_loop)]
fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        panic!("{}", e);
    }
}