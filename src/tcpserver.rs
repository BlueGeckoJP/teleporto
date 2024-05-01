use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::exit,
    thread,
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

            println!("Connection established.");

            thread::spawn(move || handle_client(&stream));
        }
    }
}

fn handle_client(mut s: &TcpStream) {
    // Recv filename
    let mut u8_filename = [0; 512];
    s.read(&mut u8_filename).unwrap();

    // Send OK
    s.write_all("OK\n".as_bytes()).unwrap();
    s.flush().unwrap();

    // Recv file
    let mut u8_file = [0; 512];
    s.read(&mut u8_file).unwrap();

    // Send OK
    s.write_all("OK\n".as_bytes()).unwrap();
    s.flush().unwrap();

    println!(
        "{} {}",
        String::from_utf8(u8_filename.to_vec()).unwrap(),
        String::from_utf8(u8_file.to_vec()).unwrap()
    );
}
