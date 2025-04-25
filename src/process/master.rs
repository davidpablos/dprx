use crate::process::worker::Worker;
use mio::net::TcpListener as MioTcpListener;
use nix::unistd::{fork, ForkResult};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{SocketAddr, TcpListener};
use std::process;

pub struct Master {
    listener: TcpListener,
    num_workers: usize,
}

impl Master {
    pub fn new(ip: String, port: u16, num_workers: usize) -> Self {
        let address = format!("{ip}:{port}");
        let listener = create_reuse_address_socket(&address);
        listener
            .set_nonblocking(true)
            .expect("Failed to set listener to non-blocking mode");

        Master {
            listener,
            num_workers,
        }
    }

    pub fn start_workers(&self) {
        for _ in 0..self.num_workers {
            match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    println!("Child PID {}: starting worker", process::id());

                    let std_listener = self
                        .listener
                        .try_clone()
                        .expect("Failed to clone TcpListener");
                    let mio_listener =
                        MioTcpListener::from_std(std_listener);

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

fn create_reuse_address_socket(addr: &str) -> TcpListener {
    let address: SocketAddr = addr.parse().expect("Invalid address");

    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))
        .expect("Failed to create socket");

    #[cfg(not(windows))]
    socket.set_reuse_address(true).unwrap();

    socket.bind(&address.into()).expect("Bind failed");
    socket.listen(128).expect("Listen failed");

    socket.into()
}
