use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};

pub mod client;
pub mod server;

async fn stream_data<In: From<Vec<u8>>, Out: Into<Vec<u8>>>(
    mut stream: TcpStream,
    mut sender: Sender<In>,
    mut receiver: Receiver<Out>,
) -> Result<(), std::io::Error> {
    let mut buffer = [0; 1 << 16];

    loop {
        select! {
            Some(out_message) = receiver.recv() => {
                let bytes: Vec<u8> = out_message.into();
                stream.write_u32(bytes.len() as u32).await;
                stream.write(&bytes).await;
            }
            in_message_len = stream.read_u32() => {
                let len = in_message_len? as usize;
                stream.read_exact(&mut buffer[0..len]).await;
                let message = In::from(Vec::<u8>::from(&buffer[..]));
                sender.send(message).await;
            }
        }
    }

    Ok(())
}

async fn stream_data2<'a, In: From<&'a [u8]> + 'static + ?Sized, Out: Into<Vec<u8>>>(
    mut stream: TcpStream,
    mut sender: Sender<In>,
    mut receiver: Receiver<Out>,
) -> Result<(), std::io::Error> {
    let mut buffer = [0; 1 << 16];

    loop {
        select! {
            Some(out_message) = receiver.recv() => {
                let bytes: Vec<u8> = out_message.into();
                stream.write_u32(bytes.len() as u32).await;
                stream.write(&bytes).await;
            }
            in_message_len = stream.read_u32() => {
                let len = in_message_len? as usize;
                // stream.read_exact(&mut buffer[0..len]).await;
                let v = In::from(&buffer[..]);
                //let message = In::from(&buffer[..]);
                //sender.send(message).await;
            }
        }
    }

    Ok(())
}
