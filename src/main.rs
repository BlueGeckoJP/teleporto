mod tcpclient;
mod tcpserver;

use std::thread;

use tcpclient::tcpclient;
use tcpserver::tcpserver;

fn main() {
    thread::spawn(tcpserver);
    thread::spawn(tcpclient);

    loop {}
}
