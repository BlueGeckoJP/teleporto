mod tcpserver;

use tcpserver::tcpserver;

use std::{
    io::{BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
    thread,
    time::Duration,
};

fn main() {
    thread::spawn(tcpserver);

    loop {
        let server_addr = "127.0.0.1:7878";
        let socket = TcpStream::connect(server_addr);
        if let Err(_) = socket {
            thread::sleep(Duration::from_secs(3));
            continue;
        }
        let socket = socket.unwrap();
        println!("Connected to server.");

        let mut reader = BufReader::new(&socket);
        let mut writer = BufWriter::new(&socket);

        let message = "Hello, world!\n".as_bytes();
        if let Ok(()) = writer.write_all(message) {
            println!("send message!!");
        }
    }
}
