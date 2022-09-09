use bincode::config::{standard, Configuration, Fixint, LittleEndian};
use bincode::{Decode, Encode};
use byteorder::{ReadBytesExt, WriteBytesExt};
use log::{info, warn};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub mod client;
pub mod server;

const BINCODE_CONFIG: Configuration<LittleEndian, Fixint> = standard()
    .with_fixed_int_encoding()
    .write_fixed_array_length();

fn stream_data<In: Decode + Send + 'static, Out: Encode + Send + 'static>(
    mut stream: TcpStream,
    sender: SyncSender<In>,
    receiver: Receiver<Out>,
) -> (std::io::Error, SyncSender<In>, Receiver<Out>) {
    let mut buffer = [0; 1 << 16];

    let mut stream_clone = stream.try_clone().unwrap();

    let t1 = thread::spawn(move || {
        let mut bytes_write = 0;
        let mut start_time = Instant::now();
        loop {
            loop {
                let v = receiver.try_recv();
                match v {
                    Ok(out_message) => {
                        let mut bytes =
                            bincode::encode_to_vec(out_message, BINCODE_CONFIG).unwrap();
                        //bytes.resize(ALIGN, 0);
                        if let Err(e) =
                            stream.write_u32::<byteorder::LittleEndian>(bytes.len() as u32)
                        {
                            return (e, receiver);
                        }
                        if let Err(e) = stream.write(&bytes) {
                            return (e, receiver);
                        }

                        bytes_write += bytes.len();
                    }
                    Err(TryRecvError::Empty) => {
                        break;
                    }
                    Err(TryRecvError::Disconnected) => {
                        panic!("Unrecoverable error :( Streaming channel was closed");
                    }
                }
            }

            sleep(Duration::from_millis(2));

            let duration = Instant::now() - start_time;
            if duration.as_millis() > 3000 {
                info!(
                    "Write throughput: {}kbps",
                    bytes_write as f32 * 8.0 / duration.as_millis() as f32
                );
                start_time = Instant::now();
                bytes_write = 0;
            }
        }
    });

    let t2 = thread::spawn(move || {
        let mut bytes_read = 0;
        let mut start_time = Instant::now();
        loop {
            match stream_clone.read_u32::<byteorder::LittleEndian>() {
                Ok(len) => match stream_clone.read_exact(&mut buffer[0..len as usize]) {
                    Ok(_) => {
                        bytes_read += len;
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
                    Err(e) => {
                        return (e, sender);
                    }
                },
                Err(e) => {
                    return (e, sender);
                }
            }

            let duration = Instant::now() - start_time;
            if duration.as_millis() > 3000 {
                info!(
                    "Read throughput: {}kbps",
                    bytes_read as f32 * 8.0 / duration.as_millis() as f32
                );
                start_time = Instant::now();
                bytes_read = 0;
            }
        }
    });

    let (e, r) = t1.join().unwrap();
    let (e, s) = t2.join().unwrap();

    (e, s, r)
}
