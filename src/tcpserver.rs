use std::{
    io::{BufReader, BufWriter, Read},
    net::TcpListener,
    process::exit,
};

fn tcpserver() {
    let listener = TcpListener::bind("localhost:8000");
    if let Err(e) = listener {
        println!("Failed to bind. {}", e);
        exit(1);
    }
    let listener = listener.unwrap();

    loop {
        println!("Waiting for connection...");
        let stream = listener.accept();
        if let Err(e) = stream {
            println!("Failed to accept. {}", e);
            exit(1);
        }
        let stream = stream.unwrap();

        println!("Connected to {}", stream.1);
        let mut stream = stream.0;
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
