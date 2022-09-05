use std::sync::Mutex;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};

use bevy::prelude::*;

use common::message::{ServerMessageData, UserMessageData};

pub struct ServerConnectionPlugin;

pub struct ServerConnection {
    pub sender: Mutex<SyncSender<UserMessageData>>,
    pub receiver: Mutex<Receiver<ServerMessageData>>,
}

pub struct PingTimer(Timer);

pub(crate) type UserMessages<'w, 's> = EventWriter<'w, 's, UserMessageData>;
pub(crate) type ServerMessages<'w, 's> = EventReader<'w, 's, ServerMessageData>;

impl Default for PingTimer {
    fn default() -> Self {
        PingTimer(Timer::from_seconds(1.0, true))
    }
}

impl Plugin for ServerConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<ServerMessageData>>();
        app.init_resource::<Events<UserMessageData>>();
        app.init_resource::<PingTimer>();

        // Receive all messages from server connections and fire events.
        app.add_system_to_stage(CoreStage::First, process_server_messages);

        // Consume all user events and send them to server.
        app.add_system_to_stage(CoreStage::Last, process_user_messages);

        // Ping!
        app.add_system(ping);
    }
}

fn ping(
    connection: Option<ResMut<ServerConnection>>,
    mut timer: ResMut<PingTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if let Some(connection) = connection {
        if timer.0.just_finished() {
            connection
                .sender
                .lock()
                .unwrap()
                .send(UserMessageData::Ping.into())
                .unwrap();
        }
    }
}

fn process_server_messages(
    connection: Option<ResMut<ServerConnection>>,
    mut event_writer: EventWriter<ServerMessageData>,
) {
    if let Some(connection) = connection {
        loop {
            match connection.receiver.lock().unwrap().try_recv() {
                Ok(message) => {
                    event_writer.send(message);
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

fn process_user_messages(
    connection: Option<ResMut<ServerConnection>>,
    mut event_reader: EventReader<UserMessageData>,
) {
    if let Some(connection) = connection {
        for message in event_reader.iter() {
            connection
                .sender
                .lock()
                .unwrap()
                .send(message.clone())
                .unwrap();
        }
    }
}
