use std::{
    io::{BufReader, BufWriter, Read},
    net::TcpListener,
    process::exit,
};

pub fn tcpserver() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    if let Err(e) = listener {
        println!("Failed to bind. {}", e);
        exit(1);
    }
    let listener = listener.unwrap();

    loop {
        println!("Waiting for connection...");
        for stream in listener.incoming() {
            if let Err(e) = stream {
                println!("Failed to accept. {}", e);
                exit(1);
            }
            let stream = stream.unwrap();

            println!("Connected to {}", stream.peer_addr().unwrap());
            let mut reader = BufReader::new(&stream);
            let mut writer = BufWriter::new(&stream);

            let mut buf = String::new();
            let size = reader.read_to_string(&mut buf);
            if let Err(e) = size {
                println!("Failed to read. {}", e);
                exit(1);
            }
            let size = size.unwrap();
            if size == 0 {
                continue;
            }
            println!("{}", buf);
        }
    }
}
