use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Worker {
    pub pid: u32,
    pub stream: TcpStream,
}

impl Worker {
    pub fn new(pid: u32, stream: TcpStream) -> Self {
        Worker { pid, stream }
    }

    pub fn handle_client(&mut self) {
        let mut buffer = [0; 512];
        match self.stream.read(&mut buffer) {
            Ok(_) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer));

                let response = "HTTP/1.1 200 OK\r\nContent-Length: 14\r\n\r\nHello world!\r\n";
                self.stream.write_all(response.as_bytes()).unwrap();
            },
            Err(e) => eprintln!("Error reading: {}", e),
        }
    }
}