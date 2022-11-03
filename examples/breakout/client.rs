use std::collections::HashMap;

use bevy::prelude::{warn, EventReader, ResMut, State};
use bevy_quinnet::{
    client::{CertificateVerificationMode, Client, ClientConfigurationData, ConnectionEvent},
    ClientId,
};

use crate::{protocol::ServerMessage, GameState};

#[derive(Debug, Clone, Default)]
pub(crate) struct Users {
    self_id: ClientId,
    names: HashMap<ClientId, String>,
}

pub(crate) fn handle_server_messages(
    mut client: ResMut<Client>,
    mut users: ResMut<Users>,
    mut game_state: ResMut<State<GameState>>,
) {
    while let Ok(Some(message)) = client.receive_message::<ServerMessage>() {
        match message {
            // ServerMessage::ClientConnected { client_id: _ } => {}
            // ServerMessage::ClientDisconnected { client_id } => {
            //     if let Some(username) = users.names.remove(&client_id) {
            //         println!("{} left", username);
            //     } else {
            //         warn!("ClientDisconnected for an unknown client_id: {}", client_id)
            //     }
            // }
            ServerMessage::InitClient { client_id } => {
                users.self_id = client_id;
            }
            ServerMessage::BrickDestroyed {} => todo!(),
            ServerMessage::BallPosition {} => todo!(),
            ServerMessage::PaddlePosition {} => todo!(),
            ServerMessage::GameStart {} => game_state.set(GameState::Running).unwrap(),
        }
    }
}

pub(crate) fn start_connection(client: ResMut<Client>) {
    client
        .connect(
            ClientConfigurationData::new("127.0.0.1".to_string(), 6000, "0.0.0.0".to_string(), 0),
            CertificateVerificationMode::SkipVerification,
        )
        .unwrap();
}
