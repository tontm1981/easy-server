use std::io::{Error, Read, self, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use super::{application::{Application, FunctionList}, request::Request};

pub struct SingleServer(TcpListener, Application);
impl SingleServer {
    pub fn try_new(host: &str, port: u16) -> Result<Self, Error> {
        let address = format!("{host}:{port}");
        let listener = TcpListener::bind(address).expect("Couldn't up the server");
        let handlers: Application = Application::new();
        Ok(Self(listener, handlers))
    }

    pub fn connect(&mut self, route: String, f: FunctionList) {
        self.1.connect(route, f);
    }

    pub fn delete(&mut self, route: String, f: FunctionList) {
        self.1.delete(route, f);
    }

    pub fn get(&mut self, route: String, f: FunctionList) {
        self.1.get(route, f);
    }

    pub fn head(&mut self, route: String, f: FunctionList) {
        self.1.head(route, f);
    }

    pub fn options(&mut self, route: String, f: FunctionList) {
        self.1.options(route, f);
    }

    pub fn patch(&mut self, route: String, f: FunctionList) {
        self.1.patch(route, f);
    }

    pub fn post(&mut self, route: String, f: FunctionList) {
        self.1.post(route, f);
    }

    pub fn put(&mut self, route: String, f: FunctionList) {
        self.1.put(route, f);
    }

    pub fn trace(&mut self, route: String, f: FunctionList) {
        self.1.trace(route, f);
    }

    pub fn run(&self) -> Result<(), Error> {
        for stream in self.0.incoming() {
            let stream = stream.unwrap();
            self.handle(stream).expect("Unable to handle request");
        }
        Ok(())
    }

    fn handle(&self, mut stream: TcpStream) -> Result<(), Error> {
        let request_string = self.read_incoming_stream(&mut stream);
        let request = self.build_request(request_string);
        let response = self.1.handle_request(request);
        self.write_response(response, stream)
    }

    fn read_incoming_stream(&self, stream: &mut TcpStream) -> String {
        let mut vec_u8: Vec<u8> = vec![];
        stream.set_read_timeout(Some(Duration::from_millis(75))).expect("Unable to set read timeout");
        loop {
            match stream.read_to_end(&mut vec_u8) {
                Ok(_) => {},
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => panic!("Failed to read incoming stream: {}", e),
            }
        }
        String::from_utf8(vec_u8).expect("Failed to convert request to string")
    }

    fn build_request(&self, input: String) -> Request {
        let parsed_request = self.parse_request_string(input);
        let request = self.build_request_instance(parsed_request[0].to_owned());
        println!("{parsed_request:#?}");
        println!("{request:#?}");
        request
    }

    fn build_request_instance(&self, input: String) -> Request {
        let parsed_string: Vec<String> = input
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        Request::new(
            parsed_string[0].to_owned(), 
            parsed_string[1].to_owned(), 
            parsed_string[2].to_owned()
        )
    }

    fn parse_request_string(&self, input: String) -> Vec<String> {
        let splitted_string: Vec<String> = input
            .split("\r\n")
            .map(|s| s.to_string())
            .collect();
        splitted_string
    }

    pub fn pub_parse_request_string(&self, input: String) -> Vec<String> {
        self.parse_request_string(input)
    }

    fn write_response(&self, response: String, mut stream: TcpStream) -> Result<(), Error> {
        stream.write_all(response.as_bytes()).expect("Unable to send response to client");
        Ok(())
    }
}