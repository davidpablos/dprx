mod process;

use crate::process::master::Master;
use std::io::{Read, Write};
use std::process as std_process;

fn main() {
    let master = Master::new(String::from("127.0.0.1"), 9000, 3);
    master.start_workers();

    println!("Parent PID {}: forked all children", std_process::id());
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
