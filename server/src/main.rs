
// https://github.com/lucaspoffo/renet/tree/master/bevy_renet

use bevy::prelude::*;
use bevy_renet::{
    renet::{
        ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig, RenetError, RenetServer, ServerAuthentication,
        ServerConfig, ServerEvent,
    },
    run_if_client_connected, RenetClientPlugin, RenetServerPlugin,
};


// Systems

fn send_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
     // Send a text message for all clients
    server.broadcast_message(channel_id, "server message".as_bytes().to_vec());
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
     // Send a text message for all clients
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, channel_id) {
            // Handle received message
        }
    }
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    while let Some(event) = server.get_event() {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                println!("Client {} connected", id);
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("Client {} disconnected", id);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut app = App::new();
    app.add_plugin(RenetServerPlugin::default());

    let server = RenetServer::new(...);
    app.insert_resource(server);

    app.add_system(send_message_system);
    app.add_system(receive_message_system);
    app.add_system(handle_events_system);
}
