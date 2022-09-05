use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::sleep;
use crate::stream_data;

pub enum ConnectionEvent<In, Out> {
    Connected(Receiver<In>, Sender<Out>),
    Disconnected,
}

pub fn start_tcp_server<In: From<Vec<u8>> + 'static + Sync + Send, Out: Into<Vec<u8>> + 'static + Sync + Send>(port: &str) -> Receiver<ConnectionEvent::<In, Out>> {
    let (connection_sender, connection_receiver) = channel(1024);

    let address = format!("0.0.0.0:{}", port);

    tokio::spawn(async move {
        loop {
            if let Ok(socket) = TcpListener::bind(&address).await {
                loop {
                    match socket.accept().await {
                        Ok((stream, _)) => {
                            let user_to_server = channel::<In>(1024);
                            let server_to_user = channel::<Out>(1024);

                            connection_sender.send(ConnectionEvent::Connected(user_to_server.1, server_to_user.0)).await;

                            tokio::spawn(async {
                                stream_data(stream, user_to_server.0, server_to_user.1);
                            });
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
            }

            sleep(Duration::from_secs(5)).await;
        }
    });

    connection_receiver
}
