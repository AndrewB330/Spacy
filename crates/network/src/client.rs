use crate::stream_data;
use bincode::{Decode, Encode};
use log::info;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread::sleep;
use std::time::Duration;

pub fn resilient_tcp_client<In: Decode + Send + 'static, Out: Encode + Send + 'static>(
    host: &str,
    port: &str,
    sender: SyncSender<In>,
    receiver: Receiver<Out>,
) {
    let mut sender_slot = Some(sender);
    let mut receiver_slot = Some(receiver);

    loop {
        match TcpStream::connect(&format!("{}:{}", host, port)) {
            Ok(stream) => {
                info!("Resilient TCP client started");
                let (e, sender, receiver) = stream_data(
                    stream,
                    sender_slot.take().unwrap(),
                    receiver_slot.take().unwrap(),
                );
                sender_slot = Some(sender);
                receiver_slot = Some(receiver);
                info!("Resilient TCP client stopped, error: {}", e);
            }
            Err(e) => {
                info!("Resilient TCP client failed to start, error: {}", e);
            }
        };

        info!("Restarting after 5 second...");
        sleep(Duration::from_secs(5));
    }
}
