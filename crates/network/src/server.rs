use crate::stream_data;
use bincode::{Decode, Encode};
use log::info;
use std::net::TcpListener;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub enum ConnectionEvent<In, Out> {
    Connected(u32, Receiver<In>, SyncSender<Out>),
    Disconnected(u32),
}

pub fn resilient_tcp_server<In: Decode + Send + 'static, Out: Encode + Send + 'static>(
    port: &str,
    connection_sender: SyncSender<ConnectionEvent<In, Out>>,
) {
    let mut counter = 0;

    loop {
        match TcpListener::bind(&format!("0.0.0.0:{}", port)) {
            Ok(listener) => {
                info!("Resilient TCP server started");
                loop {
                    match listener.accept() {
                        Ok((stream, address)) => {
                            let (in_sender, in_receiver) = sync_channel(512 * 32);
                            let (out_sender, out_receiver) = sync_channel(512 * 32);

                            let connection_id = counter;
                            counter += 1;

                            info!("Connected: {} (id: {})", address, connection_id);

                            match connection_sender.send(ConnectionEvent::Connected(
                                connection_id,
                                in_receiver,
                                out_sender,
                            )) {
                                Ok(()) => {}
                                Err(_) => {
                                    info!("Resilient TCP server stopped. Connection channel was closed, unrecoverable.");
                                    return;
                                }
                            }

                            let connection_sender_clone = connection_sender.clone();
                            thread::spawn(move || {
                                let (e, _, _) = stream_data(stream, in_sender, out_receiver);
                                info!(
                                    "Disconnected: {} (id: {}), error: {}",
                                    address, connection_id, e
                                );
                                match connection_sender_clone
                                    .send(ConnectionEvent::Disconnected(connection_id))
                                {
                                    Ok(()) => {}
                                    Err(_) => {
                                        info!("Resilient TCP server stopped. Connection channel was closed, unrecoverable.");
                                        return;
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            info!("Resilient TCP server stopped, error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                info!("Resilient TCP server failed to start, error: {}", e);
            }
        }

        info!("Restarting after 5 second...");
        sleep(Duration::from_secs(5));
    }
}
