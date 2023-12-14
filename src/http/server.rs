use std::io::{Error, Read, self, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

pub struct SingleServer(TcpListener);
impl SingleServer {
    pub fn try_new(host: &str, port: u16) -> Result<Self, Error> {
        let address = format!("{host}:{port}");
        let listener = TcpListener::bind(address).expect("Couldn't up the server");
        Ok(Self(listener))
    }

    pub fn run(&self) -> Result<(), Error> {
        for stream in self.0.incoming() {
            let stream = stream.unwrap();
            self.handle(stream).expect("Unable to handle request");
        }
        Ok(())
    }

    fn handle(&self, mut stream: TcpStream) -> Result<(), Error> {
        let mut vec_u8: Vec<u8> = vec![];
        stream.set_read_timeout(Some(Duration::from_millis(75))).expect("Unable to set read timeout");
        loop {
            match stream.read_to_end(&mut vec_u8) {
                Ok(_) => {},
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => panic!("Failed to read incoming stream: {}", e),
            }
        }
        let request = String::from_utf8(vec_u8).expect("Failed to convert request to string");
        println!("Request: {request:#?}");

        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).expect("Unable to send response to client");
        Ok(())
    }
}