use std::io::{Read, Write};
use std::net::{TcpListener as StdTcpListener};
use std::process;
use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};

const LISTENER: Token = Token(0);

pub struct Worker {
    pid: u32,
    listener: TcpListener,
}

impl Worker {
    pub fn new(pid: u32, listener: StdTcpListener) -> Self {
        let mio_listener = TcpListener::from_std(listener);
        Worker { pid, listener: mio_listener }
    }

    pub fn run(&mut self) {
        let mut poll = Poll::new().unwrap();
        poll.registry()
            .register(&mut self.listener, LISTENER, Interest::READABLE)
            .unwrap();

        let mut events = Events::with_capacity(128);
        loop {
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                match event.token() {
                    LISTENER if event.is_readable() => {
                        match self.listener.accept() {
                            Ok((stream, addr)) => {
                                println!(
                                    "PID {}: accepted connection from {}",
                                    process::id(),
                                    addr
                                );
                                self.handle_client(stream);
                            }
                            Err(e) => {
                                eprintln!("PID {}: accept failed: {}", process::id(), e);
                            }
                        }
                    },
                    _ => unreachable!(),
                }
            }
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer));

                let response = "HTTP/1.1 200 OK\r\nContent-Length: 14\r\n\r\nHello world!\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            },
            Err(e) => eprintln!("Error reading: {}", e),
        }
    }
}