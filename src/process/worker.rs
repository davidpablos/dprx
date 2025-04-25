use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::process;

const LISTENER: Token = Token(0);
const MAX_CLIENTS: usize = 1024;

pub struct Worker {
    pid: u32,
    listener: TcpListener,
}

impl Worker {
    pub fn new(pid: u32, listener: TcpListener) -> Self {
        Worker { pid, listener }
    }

    pub fn run(&mut self) {
        let mut poll = Poll::new().unwrap();

        poll.registry()
            .register(&mut self.listener, LISTENER, Interest::READABLE)
            .unwrap();

        let mut events = Events::with_capacity(128);
        let mut unique_token = 1;
        let mut connections: HashMap<Token, TcpStream> = HashMap::new();

        loop {
            poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    LISTENER => {
                        loop {
                            match self.listener.accept() {
                                Ok((mut stream, addr)) => {
                                    println!(
                                        "PID {}: accepted connection from {}",
                                        process::id(),
                                        addr
                                    );

                                    let token = Token(unique_token);
                                    unique_token += 1;
                                    if unique_token >= MAX_CLIENTS {
                                        unique_token = 1; // Avoid token overflow
                                    }

                                    poll.registry()
                                        .register(&mut stream, token, Interest::READABLE)
                                        .unwrap();

                                    connections.insert(token, stream);
                                }
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    break; // No more clients to accept
                                }
                                Err(e) => {
                                    eprintln!("accept failed: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    token => {
                        if let Some(mut stream) = connections.remove(&token) {
                            let mut buffer = [0; 512];
                            match stream.read(&mut buffer) {
                                Ok(0) => {
                                    // Client closed connection
                                }
                                Ok(n) => {
                                    println!(
                                        "PID {}: Received: {}",
                                        process::id(),
                                        String::from_utf8_lossy(&buffer[..n])
                                    );

                                    let body = "Hello world!\r\n";
                                    let content_length = body.len();

                                    let response = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                        content_length,
                                        body
                                    );
                                    stream.write_all(response.as_bytes()).ok();
                                    stream.flush().ok();
                                }
                                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    // Not ready yet, reinsert connection
                                    connections.insert(token, stream);
                                    continue;
                                }
                                Err(e) => {
                                    eprintln!("read failed: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}