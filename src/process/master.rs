use crate::process::worker::Worker;
use nix::unistd::{fork, ForkResult};
use std::net::TcpListener;
use std::process;

pub struct Master {
    listener: TcpListener,
    num_workers: usize,
}

impl Master {
    pub fn new(ip: String, port: u16, num_workers: usize) -> Self {
        let address = ip + ":" + &port.to_string();
        let listener = TcpListener::bind(address).expect("Failed to bind to address");
        listener
            .set_nonblocking(true)
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
                    let std_listener = self.listener.try_clone().unwrap();
                    let mio_listener = mio::net::TcpListener::from_std(std_listener);
                    let mut worker = Worker::new(process::id(), mio_listener);
                    worker.run();
                    process::exit(0);
                }
                Ok(ForkResult::Parent { .. }) => continue,
                Err(e) => eprintln!("fork failed: {}", e),
            }
        }
    }
}
