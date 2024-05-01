use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

pub fn tcpclient() {
    loop {
        let server_addr = "127.0.0.1:7878";
        let socket = TcpStream::connect(server_addr);
        if let Err(_) = socket {
            thread::sleep(Duration::from_secs(3));
            continue;
        }
        let mut socket = socket.unwrap();
        println!("Connected to server.");

        // Send filename
        socket.write_all("A".as_bytes()).unwrap();
        socket.flush().unwrap();

        // Recv OK
        let mut ok: Vec<u8> = vec![];
        socket.read(&mut ok).unwrap();

        // Send file
        socket.write_all("B".as_bytes()).unwrap();
        socket.flush().unwrap();

        // Recv OK
        socket.read(&mut ok).unwrap();

        break;
    }
}
