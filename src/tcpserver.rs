use std::{
    io::{BufReader, BufWriter, Read, Write},
    iter,
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

fn handle_client(s: &TcpStream) {
    let mut reader = BufReader::new(s);
    let mut writer = BufWriter::new(s);

    // Recv filename
    let mut u8_filename = [0; 512];
    reader.read(&mut u8_filename).unwrap();

    // Send OK
    writer.write_all("OK".as_bytes()).unwrap();
    writer.flush().unwrap();

    // Recv file
    let mut file_array: Vec<[u8; 512]> = vec![];
    loop {
        let mut u8_file = [0; 512];
        let r = reader.read(&mut u8_file);
        if let Err(_) = r {
            break;
        }
        file_array.push(u8_file);
    }

    println!("{:?}", vec_u8_512_to_string(file_array));
}

fn vec_u8_512_to_string(arr: Vec<[u8; 512]>) -> String {
    let mut out: Vec<u8> = vec![];
    for v in arr.iter() {
        for u8v in v.iter() {
            if u8v.clone() != 0 {
                out.push(u8v.clone());
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}
