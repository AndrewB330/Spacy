use crate::stream_data;
use bincode::{Decode, Encode};
use log::info;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use std::sync::mpsc::{channel as channel_s, Receiver as ReceiverS, Sender as SenderS};
use tokio::time::sleep;

pub enum ConnectionEvent<In, Out> {
    Connected(u32, Receiver<In>, SenderS<Out>),
    Disconnected(u32),
}

#[tokio::main]
pub async fn resilient_tcp_server<In: Decode + Send + 'static, Out: Encode + Send + 'static>(
    port: &str,
    connection_sender: Sender<ConnectionEvent<In, Out>>,
) {
    let mut counter = 0;

    loop {
        match TcpListener::bind(&format!("0.0.0.0:{}", port)).await {
            Ok(listener) => {
                info!("Resilient TCP server started");
                loop {
                    match listener.accept().await {
                        Ok((stream, address)) => {
                            info!("Connected: {}", address);

                            let (mut in_sender, in_receiver) = channel::<In>(1024);
                            let (out_sender, mut out_receiver) = channel_s::<Out>();

                            let connection_id = counter;
                            counter += 1;

                            match connection_sender
                                .send(ConnectionEvent::Connected(
                                    connection_id,
                                    in_receiver,
                                    out_sender,
                                ))
                                .await
                            {
                                Ok(()) => {}
                                Err(_) => {
                                    info!("Resilient TCP server stopped. Connection channel was closed, unrecoverable.");
                                    return;
                                }
                            }

                            let connection_sender_clone = connection_sender.clone();
                            tokio::spawn(async move {
                                if let Err(e) =
                                    stream_data(stream, &mut in_sender, &mut out_receiver).await
                                {
                                    info!("Disconnected: {}, error: {}", address, e);
                                    match connection_sender_clone
                                        .send(ConnectionEvent::Disconnected(connection_id))
                                        .await
                                    {
                                        Ok(()) => {}
                                        Err(_) => {
                                            info!("Resilient TCP server stopped. Connection channel was closed, unrecoverable.");
                                            return;
                                        }
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
        sleep(Duration::from_secs(5)).await;
    }
}
