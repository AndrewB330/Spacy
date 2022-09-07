use bincode::config::{standard, Configuration, Fixint, LittleEndian};
use bincode::{Decode, Encode};
use log::warn;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::time::sleep;

pub mod client;
pub mod server;

const BINCODE_CONFIG: Configuration<LittleEndian, Fixint> = standard()
    .with_fixed_int_encoding()
    .write_fixed_array_length();

const ALIGN: usize = 128;

async fn stream_data<In: Decode, Out: Encode>(
    mut stream: TcpStream,
    sender: SyncSender<In>,
    receiver: Receiver<Out>,
) -> (std::io::Error, SyncSender<In>, Receiver<Out>) {
    let mut buffer = [0; 1 << 16];

    loop {
        select! {
            _ = sleep(Duration::from_millis(2)) => {
                loop {
                    let v = receiver.try_recv();
                    match v {
                        Ok(out_message) => {
                            let mut bytes = bincode::encode_to_vec(out_message, BINCODE_CONFIG).unwrap();
                            bytes.resize(ALIGN, 0);
                            //if let Err(e) = stream.write_u32(bytes.len() as u32).await {
                            //    return (e, sender, receiver);
                            //}*
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

            in_message_len = stream.read_exact(&mut buffer[0..ALIGN]) => {
                match bincode::decode_from_slice(&buffer, BINCODE_CONFIG) {
                    Ok((message, _)) => {
                        if let Err(_) = sender.send(message) {
                            panic!("Unrecoverable error :( Streaming channel was closed");
                        }
                    }
                    Err(e) => {
                        warn!("Corrupted message! Error: {}", e);
                    }
                }
            }
        }
    }
}
