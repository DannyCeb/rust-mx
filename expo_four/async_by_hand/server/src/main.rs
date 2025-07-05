use std::{
    io::{self, Cursor, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, channel},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use async_runtime::{executor::Executor, sleep::Sleep};
use data_layer::data::Data;

static FLAGS: [AtomicBool; 3] = [
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
];

fn spawn_worker(
    name: &'static str,
    rx: Receiver<TcpStream>,
    flag: &'static AtomicBool,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut executor = Executor::new();
        loop {
            if let Ok(stream) = rx.try_recv() {
                println!(
                    "{} Received connection {}",
                    name,
                    stream.peer_addr().unwrap()
                );
                executor.spawn(handle_client(stream));
            } else {
                if executor.polling.len() == 0 {
                    println!("{} is sleeping", name);
                    flag.store(true, Ordering::SeqCst);
                    thread::park();
                }

                executor.poll();
            }
        }
    })
}

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_nonblocking(true)?;
    let mut buffer = Vec::new();

    let mut local_buf = [0; 1024];

    loop {
        match stream.read(&mut local_buf) {
            Ok(0) => break,
            Ok(len) => {
                buffer.extend_from_slice(&local_buf[..len]);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                if buffer.len() > 0 {
                    break;
                }

                Sleep::new(Duration::from_millis(10)).await;
                continue;
            }
            Err(e) => {
                println!("Failed to read from connection: {}", e);
            }
        }
    }

    match Data::deserialize(&mut Cursor::new(buffer.as_slice())) {
        Ok(message) => {
            println!("Received message: {:?}", message);
        }
        Err(e) => {
            println!("Failed to decode message: {}", e);
        }
    }

    Sleep::new(Duration::from_secs(1)).await;
    stream.write_all(b"Hello, Client!")?;
    Ok(())
}

fn main() -> io::Result<()> {
    let (one_tx, one_rx) = channel::<TcpStream>();
    let (two_tx, two_rx) = channel::<TcpStream>();
    let (three_tx, three_rx) = channel::<TcpStream>();

    let worker_one: JoinHandle<()> = spawn_worker("One", one_rx, &FLAGS[0]);
    let worker_two: JoinHandle<()> = spawn_worker("two", two_rx, &FLAGS[1]);
    let worker_three: JoinHandle<()> = spawn_worker("three", three_rx, &FLAGS[2]);

    let router = [one_tx, two_tx, three_tx];
    let threads = [worker_one, worker_two, worker_three];

    let mut index: usize = 0;

    let listener = TcpListener::bind("0.0.0.0:7878")?;

    println!("Server Listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = router[index].send(stream);

                if FLAGS[index].load(Ordering::SeqCst) {
                    FLAGS[index].store(false, Ordering::SeqCst);
                    threads[index].thread().unpark();
                }

                index += 1;
                if index == 3 {
                    index = 0;
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e)
            }
        }
    }

    Ok(())
}
