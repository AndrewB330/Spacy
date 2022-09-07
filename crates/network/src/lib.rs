use bincode::config::{standard, Configuration};
use bincode::{Decode, Encode};
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::time::sleep;

pub mod client;
pub mod server;

const BINCODE_CONFIG: Configuration = standard();

async fn stream_data<In: Decode, Out: Encode>(
    mut stream: TcpStream,
    mut sender: SyncSender<In>,
    mut receiver: Receiver<Out>,
) -> (std::io::Error, SyncSender<In>, Receiver<Out>) {
    let mut buffer = [0; 1 << 16];

    loop {
        select! {
            _ = sleep(Duration::from_millis(2)) => {
                loop {
                    let v = receiver.try_recv();
                    match v {
                        Ok(out_message) => {
                        let bytes = bincode::encode_to_vec(out_message, BINCODE_CONFIG).unwrap();
                        if let Err(e) = stream.write_u32(bytes.len() as u32).await {
                            return (e, sender, receiver);
                        }
                        if let Err(e) = stream.write(&bytes).await {
                            return (e, sender, receiver);
                        }
                        }
                    Err(TryRecvError::Empty) => {
                        break;
                    }
                    Err(TryRecvError::Disconnected) => {
                        panic!("Unrecoverable error :( Streaming channel was closed");
                    }
                    }
                }
            }

            in_message_len = stream.read_u32() => {

                let len = match in_message_len {
                    Ok(v) => v as usize,
                    Err(e) => return (e, sender, receiver),
                };
                if let Err(e) = stream.read_exact(&mut buffer[0..len]).await {
                            return (e, sender, receiver);
                }
                let message = bincode::decode_from_slice(&buffer, BINCODE_CONFIG).unwrap().0;
                if let Err(_) = sender.send(message) {
                    panic!("Unrecoverable error :( Streaming channel was closed");
                }
            }
        }
    }
}
