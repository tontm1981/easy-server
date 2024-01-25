pub mod common;

use std::io::{Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::io::prelude::*;
use common::types::ApplicationHandler;

use crate::common::{Application, Request, Response};
use crate::common::enums::HttpStatuses;

pub struct Server {
    listener: TcpListener,
    application: Application
}

const STREAM_READ_TIMEOUT: u64 = 50;
const STREAM_WRITE_TIMEOUT: u64 = 50;

impl Server {
    pub fn try_new(host: &str, port: usize) -> Result<Self, Error> {
        let address = format!("{host}:{port}");
        let listener = TcpListener::bind(address);
        let application = Application::new();
        match listener {
            Ok(listener) => Ok(Self { listener, application }),
            Err(e) => Err(e)
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        for stream in self.listener.incoming() {
            let stream = stream.expect("Unable to open stream for request");
            stream.set_read_timeout(Some(Duration::from_millis(STREAM_READ_TIMEOUT))).expect("Failed to change settings on stream");
            stream.set_write_timeout(Some(Duration::from_millis(STREAM_WRITE_TIMEOUT))).expect("Failed to change settings on stream");
            let application = self.application.clone();
            Server::handle_request(stream, application);
        }
        Ok(())
    }

    pub fn connect(&mut self, route: String, function: ApplicationHandler) {
        self.application.connect(route, function);
    }

    pub fn delete(&mut self, route: String, function: ApplicationHandler) {
        self.application.delete(route, function);
    }

    pub fn get(&mut self, route: String, function: ApplicationHandler) {
        self.application.get(route, function);
    }

    pub fn head(&mut self, route: String, function: ApplicationHandler) {
        self.application.head(route, function);
    }

    pub fn options(&mut self, route: String, function: ApplicationHandler) {
        self.application.options(route, function);
    }

    pub fn patch(&mut self, route: String, function: ApplicationHandler) {
        self.application.patch(route, function);
    }

    pub fn post(&mut self, route: String, function: ApplicationHandler) {
        self.application.post(route, function);
    }

    pub fn put(&mut self, route: String, function: ApplicationHandler) {
        self.application.put(route, function);
    }

    pub fn trace(&mut self, route: String, function: ApplicationHandler) {
        self.application.trace(route, function);
    }

    fn get_request_as_string(mut stream: &TcpStream) -> String {
        let mut buffer: Vec<u8> = Vec::new();
        loop {
            match stream.read_to_end(&mut buffer) {
                Ok(_) => {},
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(err) => panic!("Couldn't read incoming request: {}", err),
            }
        }
        String::from_utf8(buffer).expect("Couldn't parse request")
    }

    fn build_request_instance(mut stream: &TcpStream) -> Request {
        let string = Self::get_request_as_string(&mut stream);
        Request::from_string(string)
    }

    fn build_response_instance(request: Request, mut application: Application) -> Response {
        let handler = match application.get_handler(request.method(), request.uri()) {
            None => Response::build_from_status(request.protocol(), HttpStatuses::NotFound),
            Some(f) => {
                match f(&request) {
                    Ok(result) => {
                        match result {
                            None => Response::build_from_status(request.protocol(), HttpStatuses::NoContent),
                            Some(str) => {
                                let mut response = Response::build_from_status(request.protocol(), HttpStatuses::Ok);
                                response.set_content(str);
                                response
                            }
                        }
                    }
                    Err(_) => Response::build_from_status(request.protocol(), HttpStatuses::InternalServerError),
                }
            }
        };
        handler
    }

    fn handle_request(mut stream: TcpStream, mut application: Application) {
        let request = Self::build_request_instance(&mut stream);
        let response = Self::build_response_instance(request, application);
        stream.write_all(response.to_string().as_bytes()).expect("Unable to send response to client");
        stream.flush().unwrap();
    }
}
