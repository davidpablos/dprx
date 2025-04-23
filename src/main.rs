use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
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

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
