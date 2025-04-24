use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;

pub struct Worker {
    pid: u32,
    listener: TcpListener,
}

impl Worker {
    pub fn new(pid: u32, listener: TcpListener) -> Self {
        Worker { pid, listener }
    }

    pub fn run(&self) {
        loop {
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