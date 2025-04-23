use nix::unistd::{ForkResult, fork};
use std::net::TcpListener;
use std::process;
use crate::process::worker;
use crate::process::worker::Worker;

pub struct Master {
    listener: TcpListener,
    num_workers: usize,
}

impl Master {
    pub fn new(ip: String, port: u16, num_workers: usize) -> Self {
        let address = ip + ":" + &port.to_string();
        let listener = TcpListener::bind(address).expect("Failed to bind to address");
        listener
            .set_nonblocking(false)
            .expect("Failed to set blocking mode");
        Master {
            listener,
            num_workers,
        }
    }

    pub fn start_workers(&self) {
        for _ in 0..self.num_workers {
            match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    println!("Child PID {}: listening for connections", process::id());
                    loop {
                        match self.listener.accept() {
                            Ok((stream, addr)) => {
                                println!(
                                    "PID {}: accepted connection from {}",
                                    process::id(),
                                    addr
                                );
                                let mut worker = Worker::new(process::id(), stream);
                                worker.handle_client();
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
    }
}
