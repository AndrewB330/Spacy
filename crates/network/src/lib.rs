#![feature(never_type)]

use bincode::config::{standard, Configuration};
use bincode::{Decode, Encode};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc::{channel as channel_s, Receiver as ReceiverS, Sender as SenderS};

pub mod client;
pub mod server;

const BINCODE_CONFIG: Configuration = standard();

async fn stream_data<In: Decode, Out: Encode>(
    mut stream: TcpStream,
    sender: &mut Sender<In>,
    receiver: &mut Receiver<Out>,
) -> Result<!, std::io::Error> {
    let mut buffer = [0; 1 << 16];

    loop {
        select! {
            out_message = receiver.recv() => {
                match out_message {
                    Some(out_message) => {
                        let bytes = bincode::encode_to_vec(out_message, BINCODE_CONFIG).unwrap();
                        stream.write_u32(bytes.len() as u32).await?;
                        stream.write(&bytes).await?;
                    }
                    None => {
                        panic!("Unrecoverable error :( Streaming channel was closed");
                    }
                }
            }
            in_message_len = stream.read_u32() => {
                let len = in_message_len? as usize;
                stream.read_exact(&mut buffer[0..len]).await?;
                let message = bincode::decode_from_slice(&buffer, BINCODE_CONFIG).unwrap().0;
                if let Err(_) = sender.send(message).await {
                    panic!("Unrecoverable error :( Streaming channel was closed");
                }
            }
        }
    }
}
