use nix::unistd::{fork, ForkResult};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;

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

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    listener.set_nonblocking(false).unwrap();
    for _ in 0..3 {
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                println!("Child PID {}: listening for connections", process::id());

                loop {
                    match listener.accept() {
                        Ok((stream, addr)) => {
                            println!("PID {}: accepted connection from {}", process::id(), addr);
                            handle_client(stream);
                        }
                        Err(e) => {
                            eprintln!("PID {}: accept failed: {}", process::id(), e);
                        }
                    }
                }
            }
            Ok(ForkResult::Parent { .. }) => continue,
            Err(e) => eprintln!("fork failed: {}", e),
        }
    }

    println!("Parent PID {}: forked all children", process::id());
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
