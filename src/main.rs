mod readfile;
mod tcpclient;
mod tcpserver;

use std::thread;

use readfile::readfile;
use tcpclient::{tcpclient, FileData};
use tcpserver::tcpserver;

fn main() {
    thread::spawn(tcpserver);
    thread::spawn(|| {
        let data = FileData {
            filename: String::from("A"),
            file: readfile("".to_string()),
        };
        tcpclient(data);
    });

    loop {}
}
