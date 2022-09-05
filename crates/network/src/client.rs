use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::stream_data;

pub async fn start_tcp_client<In: From<Vec<u8>>, Out: Into<Vec<u8>>>(
    host: &str,
    port: &str,
) -> (Sender<Out>, Receiver<In>) {
    let (client_sender, client_receiver) = channel::<Out>(1024);
    let (server_sender, server_receiver) = channel::<In>(1024);

    let stream = TcpStream::connect(format!("{}:{}", host, port)).await.unwrap();

    // todo: run in another thread??
    stream_data(stream, server_sender, client_receiver);

    (client_sender, server_receiver)
}
