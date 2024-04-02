use std::io::{Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::io::prelude::*;
use crate::http::server::common::traits::HttpServer;
use super::common::{Application, Request, Response, types::ApplicationHandler, enums::HttpStatuses, STREAM_WRITE_TIMEOUT, STREAM_READ_TIMEOUT, get_tcp_listener_and_application};


pub struct Server {
    listener: TcpListener,
    application: Application
}

impl Server {
    pub fn try_new(host: &str, port: usize) -> Result<Self, Error> {
        let (listener, application) = get_tcp_listener_and_application(host, port)?;
        Ok(Self { listener, application })
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

    pub fn handle_request(mut stream: TcpStream, mut application: Application) {
        let request = Self::build_request_instance(&mut stream);
        let response = Self::build_response_instance(request, application);
        stream.write_all(response.to_string().as_bytes()).expect("Unable to send response to client");
        stream.flush().unwrap();
    }
}

impl HttpServer for Server {
    fn connect(&mut self, route: String, function: ApplicationHandler) {
        self.application.connect(route, function);
    }

    fn delete(&mut self, route: String, function: ApplicationHandler) {
        self.application.delete(route, function);
    }

    fn get(&mut self, route: String, function: ApplicationHandler) {
        self.application.get(route, function);
    }

    fn head(&mut self, route: String, function: ApplicationHandler) {
        self.application.head(route, function);
    }

    fn options(&mut self, route: String, function: ApplicationHandler) {
        self.application.options(route, function);
    }

    fn patch(&mut self, route: String, function: ApplicationHandler) {
        self.application.patch(route, function);
    }

    fn post(&mut self, route: String, function: ApplicationHandler) {
        self.application.post(route, function);
    }

    fn put(&mut self, route: String, function: ApplicationHandler) {
        self.application.put(route, function);
    }

    fn trace(&mut self, route: String, function: ApplicationHandler) {
        self.application.trace(route, function);
    }
}