use crate::stream_data;
use bincode::{Decode, Encode};
use log::info;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc::{channel as channel_s, Receiver as ReceiverS, Sender as SenderS};
use tokio::time::sleep;

#[tokio::main]
pub async fn resilient_tcp_client<In: Decode + Send + 'static, Out: Encode + Send + 'static>(
    host: &str,
    port: &str,
    mut sender: Sender<In>,
    mut receiver: Receiver<Out>,
) {
    loop {
        match TcpStream::connect(&format!("{}:{}", host, port)).await {
            Ok(stream) => {
                info!("Resilient TCP client started");
                if let Err(e) = stream_data(stream, &mut sender, &mut receiver).await {
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
