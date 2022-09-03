use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::Mutex;

use bevy::prelude::*;
use bevy::utils::HashMap;

use common::message::{ServerMessage, UserId, UserMessage};

pub struct UserConnection {
    pub user_id: UserId,
    pub from_user: Mutex<Receiver<UserMessage>>,
    pub to_user: Mutex<Sender<ServerMessage>>,
}

#[derive(Default)]
pub struct UserConnections {
    pub map: HashMap<UserId, UserConnection>,
}

pub enum UserConnectionEvent {
    Connected(UserConnection),
    Disconnected(UserId),
}

pub struct UserConnectionEvents {
    pub receiver: Mutex<Receiver<UserConnectionEvent>>,
}

pub struct UserConnectionsPlugin;

pub(crate) type UserMessages<'w, 's> = EventReader<'w, 's, (UserId, UserMessage)>;
pub(crate) type ServerMessages<'w, 's> = EventWriter<'w, 's, (UserId, ServerMessage)>;

impl Plugin for UserConnectionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UserConnections>();

        app.init_resource::<Events<(UserId, ServerMessage)>>();
        app.init_resource::<Events<(UserId, UserMessage)>>();

        // Process Connect and Disconnect server events.
        app.add_system_to_stage(CoreStage::First, process_connection_events);
        // Receive all messages from user connections and fire events.
        app.add_system_to_stage(CoreStage::First, process_user_messages);
        // Consume server events and send them to user connections.
        app.add_system_to_stage(CoreStage::Last, process_server_messages);

        // Pong!
        app.add_system(pong);
    }
}

fn pong(mut server_messages: ServerMessages, mut user_messages: UserMessages) {
    for (user_id, message) in user_messages.iter() {
        match message {
            UserMessage::Ping => {
                server_messages.send((*user_id, ServerMessage::Pong));
            }
            _ => {}
        }
    }
}

fn process_connection_events(connection_events: Option<ResMut<UserConnectionEvents>>, mut connections: ResMut<UserConnections>) {
    if let Some(connection_events) = connection_events {
        let recv = connection_events.receiver.lock().unwrap().try_recv();
        match recv {
            Ok(event) => {
                match event {
                    UserConnectionEvent::Connected(connection) => connections.map.insert(connection.user_id, connection),
                    UserConnectionEvent::Disconnected(id) => connections.map.remove(&id)
                };
            }
            Err(TryRecvError::Disconnected) => {
                panic!("Unexpected end of channel!")
            }
            Err(TryRecvError::Empty) => {}
        }
    }
}

fn process_user_messages(connections: ResMut<UserConnections>, mut event_writer: EventWriter<(UserId, UserMessage)>) {
    for connection in connections.map.values() {
        loop {
            match connection.from_user.lock().unwrap().try_recv() {
                Ok(message) => {
                    event_writer.send((connection.user_id, message));
                }
                Err(TryRecvError::Empty) => { break; }
                _ => { panic!("Unexpected end of channel!") }
            }
        }
    }
}

fn process_server_messages(connections: ResMut<UserConnections>, mut event_reader: EventReader<(UserId, ServerMessage)>) {
    for (user_id, message) in event_reader.iter() {
        if let Some(connection) = connections.map.get(user_id) {
            connection.to_user.lock().unwrap().send(message.clone()).unwrap();
        }
    }
}
