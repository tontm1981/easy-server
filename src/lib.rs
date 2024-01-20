use std::io::{Error, ErrorKind};
use std::net::TcpListener;
use std::time::Duration;
use std::io::prelude::*;

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn try_new(host: &str, port: usize) -> Result<Self, Error> {
        let address = format!("{host}:{port}");
        let listener = TcpListener::bind(address);
        match listener {
            Ok(listener) => Ok(Self {listener}),
            Err(e) => Err(e)
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        for stream in self.listener.incoming() {
            let mut stream = stream.expect("Unable to open stream for request");
            stream.set_read_timeout(Some(Duration::from_millis(50))).expect("Failed to change settings on stream");
            stream.set_write_timeout(Some(Duration::from_millis(50))).expect("Failed to change settings on stream");
            let mut buffer: Vec<u8> = Vec::new();
            loop {
                match stream.read_to_end(&mut buffer) {
                    Ok(_) => {},
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
                    Err(err) => panic!("Couldn't read incoming request: {}", err),
                }
            }
            let request = String::from_utf8(buffer).expect("Couldn't parse request");
            println!("{request:#?}");
            let response = "HTTP/1.1 200 Ok\r\n\r\n";
            stream.write_all(response.as_bytes()).expect("Unable to send response to client");
            stream.flush().unwrap();
        }
        Ok(())
    }
}