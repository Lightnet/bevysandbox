
use bevy::prelude::*;
use bevy_renet::{
    renet::{
        ClientAuthentication, RenetClient, RenetConnectionConfig
    },
    RenetClientPlugin,
};

use std::time::SystemTime;
use std::{net::UdpSocket};

//use serde::{Deserialize, Serialize};

const PROTOCOL_ID: u64 = 7;

// Systems

fn send_message_system(mut client: ResMut<RenetClient>) {
    let channel_id = 0;
     // Send a text message to the server
    client.send_message(channel_id, "server message".as_bytes().to_vec());
}

fn receive_message_system(mut client: ResMut<RenetClient>) {
    let channel_id = 0;
    while let Some(message) = client.receive_message(channel_id) {
        // Handle received message
    }
}


fn main() {
    //println!("Hello, world!");
    let mut app = App::new();
    app.add_plugin(RenetClientPlugin::default());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let client = RenetClient::new(current_time, socket, connection_config, authentication).unwrap();
    //let client = RenetClient::new(...);
    app.insert_resource(client);

    app.add_system(send_message_system);
    app.add_system(receive_message_system);
}