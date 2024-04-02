use std::io::Error;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use crate::http::server::common::{Application, get_tcp_listener_and_application, STREAM_READ_TIMEOUT, STREAM_WRITE_TIMEOUT};
use crate::http::server::common::traits::HttpServer;
use crate::http::server::common::types::ApplicationHandler;
use crate::http::server::single::Server as SingleServer;

pub struct Server {
    listener: TcpListener,
    application: Application
}

impl Server {
    pub fn try_new(host: &str, port: usize) -> Result<Self, Error> {
        let (listener, application) = get_tcp_listener_and_application(host, port)?;
        Ok(Self{ listener, application })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        for stream in self.listener.incoming() {
            let stream = stream.expect("Unable to open stream for request");
            stream.set_read_timeout(Some(Duration::from_millis(STREAM_READ_TIMEOUT))).expect("Failed to change settings on stream");
            stream.set_write_timeout(Some(Duration::from_millis(STREAM_WRITE_TIMEOUT))).expect("Failed to change settings on stream");
            let application = self.application.clone();
            thread::spawn(move || {
                SingleServer::handle_request(stream, application);
            });
        }

        Ok(())
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
