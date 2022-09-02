use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::common::Message;

pub struct UserConnection {
    pub id: u32,
    pub receiver: Mutex<Receiver<Message>>,
    pub sender: Mutex<Sender<Message>>,
}

pub struct UserConnections {
    pub connections: HashMap<u32, UserConnection>,
    pub new_connections: Mutex<Receiver<UserConnection>>,
}

pub fn start_server_app(new_connections: Receiver<UserConnection>) {
    let mut app = App::new();
    app.insert_resource(UserConnections {
        connections: Default::default(),
        new_connections: Mutex::new(new_connections),
    });
    app.run();
}
