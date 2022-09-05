use crate::stream_data;
use bincode::{Decode, Encode};
use log::info;
use std::time::Duration;
use tokio::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;

#[tokio::main]
pub async fn resilient_tcp_client<In: Decode + Send + 'static, Out: Encode + Send + 'static>(
    host: &str,
    port: &str,
    mut sender: Sender<In>,
    mut receiver: Receiver<Out>,
) {
    let mut sender_slot = Some(sender);
    let mut receiver_slot = Some(receiver);

    loop {
        match TcpStream::connect(&format!("{}:{}", host, port)).await {
            Ok(stream) => {
                info!("Resilient TCP client started");
                if let Err((e, sender, receiver)) = stream_data(stream, sender_slot.take().unwrap(), receiver_slot.take().unwrap()).await {
                    sender_slot = Some(sender);
                    receiver_slot = Some(receiver);
                    info!("Resilient TCP client stopped, error: {}", e);
                }
            }
            Err(e) => {
                info!("Resilient TCP client failed to start, error: {}", e);
            }
        };

        info!("Restarting after 5 second...");
        sleep(Duration::from_secs(5)).await;
    }
}
