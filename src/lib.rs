use std::{net::{TcpListener, TcpStream}, io::prelude::*, io};
use std::time::Duration;

pub mod http;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Incoming a request.");
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    println!("handling the request");
    stream.set_read_timeout(Some(Duration::from_millis(50))).unwrap();
    let mut u8_vec: Vec<u8> = vec![];
    loop {
        match stream.read_to_end(&mut u8_vec) {
            Ok(_) => {},
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
            Err(e) => panic!("Failed when reading incoming stream: {}", e),
        }
    }
    let http_request = String::from_utf8(u8_vec).expect("Couldn't convert request to string");
    println!("{:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}