mod tcpserver;

use std::{
    io::{BufReader, BufWriter},
    net::{TcpStream, ToSocketAddrs},
};

fn main() {
    let host_and_port = "localhost:8000";
    let mut addrs = host_and_port.to_socket_addrs().unwrap();

    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
        match TcpStream::connect(addr) {
            Err(_) => {
                println!("Connection failed.");
            }
            Ok(stream) => {
                println!("Connection success.");

                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
            }
        }
    }
}
