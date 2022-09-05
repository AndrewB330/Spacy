use std::sync::Mutex;

use bevy::prelude::*;
use bevy::utils::HashMap;
use tokio::sync::mpsc::{error::TryRecvError, Receiver, Sender};

use common::message::{ServerMessageData, UserMessageData};
use common::user::UserId;
use network::server::ConnectionEvent;

pub struct UserConnection {
    pub user_id: UserId,
    pub from_user: Mutex<Receiver<UserMessageData>>,
    pub to_user: Mutex<Sender<ServerMessageData>>,
}

#[derive(Default)]
pub struct UserConnections {
    pub map: HashMap<u32, UserConnection>,
}

pub enum UserConnectionEvent {
    Connected(UserConnection),
    Disconnected(UserId),
}

pub struct UserConnectionEvents {
    pub receiver: Mutex<Receiver<ConnectionEvent<UserMessageData, ServerMessageData>>>,
}

pub struct UserConnectionsPlugin;

pub(crate) type UserMessages<'w, 's> = EventReader<'w, 's, (UserId, UserMessageData)>;
pub(crate) type ServerMessages<'w, 's> = EventWriter<'w, 's, (UserId, ServerMessageData)>;

impl Plugin for UserConnectionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UserConnections>();

        app.init_resource::<Events<(UserId, ServerMessageData)>>();
        app.init_resource::<Events<(UserId, UserMessageData)>>();

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
            UserMessageData::Ping => {
                server_messages.send((*user_id, ServerMessageData::Pong.into()));
            }
            _ => {}
        }
    }
}

fn process_connection_events(
    connection_events: Option<ResMut<UserConnectionEvents>>,
    mut connections: ResMut<UserConnections>,
) {
    if let Some(connection_events) = connection_events {
        let recv = connection_events.receiver.lock().unwrap().try_recv();
        match recv {
            Ok(event) => {
                match event {
                    ConnectionEvent::Connected(id, receiver, sender) => connections.map.insert(
                        id,
                        UserConnection {
                            user_id: UserId::new(),
                            from_user: Mutex::new(receiver),
                            to_user: Mutex::new(sender),
                        },
                    ),
                    ConnectionEvent::Disconnected(id) => connections.map.remove(&id),
                };
            }
            Err(TryRecvError::Disconnected) => {
                panic!("Unexpected end of channel!")
            }
            Err(TryRecvError::Empty) => {}
        }
    }
}

fn process_user_messages(
    connections: ResMut<UserConnections>,
    mut event_writer: EventWriter<(UserId, UserMessageData)>,
) {
    for connection in connections.map.values() {
        loop {
            match connection.from_user.lock().unwrap().try_recv() {
                Ok(message) => {
                    event_writer.send((connection.user_id, message));
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                _ => {
                    panic!("Unexpected end of channel!")
                }
            }
        }
    }
}

fn process_server_messages(
    connections: ResMut<UserConnections>,
    mut event_reader: EventReader<(UserId, ServerMessageData)>,
) {
    let mut mapping = HashMap::default();
    for (k, v) in connections.map.iter() {
        mapping.insert(v.user_id, *k);
    }

    let mut sender_copies = HashMap::new();
    let mut messages = vec![];

    for (user_id, message) in event_reader.iter() {
        if let Some(connection) = mapping.get(user_id).and_then(|id| connections.map.get(id)) {
            sender_copies.insert(*user_id, connection.to_user.lock().unwrap().clone());
        }
        messages.push((*user_id, message.clone()));
    }

    for (user_id, message) in messages {
        if let Some(sender) = sender_copies.get(&user_id) {
            sender.send(message.clone()).await.unwrap();
        }
    }
}
