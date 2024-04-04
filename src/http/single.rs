use std::{
  io::Error,
  net::{TcpListener, TcpStream},
};
use std::io::{ErrorKind, Read, Write};
use std::time::Duration;
use crate::http::common::{Application, Middleware, MiddlewareList};

pub struct Server {
  listener: TcpListener,
  handler: Application
}

impl Server {
  pub fn bind(address: &str, port: u16) -> Result<Self, Error> {
    let formatted_address = format!("{address}:{port}");
    let listener = TcpListener::bind(formatted_address)?;
    let handler = Application::default();
    Ok(Self { listener, handler })
  }

  pub fn set_middlewares(&mut self, middleware_list: MiddlewareList) {
    self.handler.set_middlewares(middleware_list)
  }

  pub fn add_middleware(&mut self, middleware: Middleware) {
    self.handler.add_middleware(middleware)
  }

  pub fn run(&self) {
    loop {
      let (mut stream, _) = self.listener.accept().expect("Could not read incoming request");
      stream.set_read_timeout(Some(Duration::from_millis(1000))).unwrap();
      stream.set_write_timeout(Some(Duration::from_millis(100))).unwrap();
      handle_request(&mut stream);
    }
  }
}

fn handle_request(stream: &mut TcpStream) {
  let raw_request = {
    let mut vector: Vec<u8> = vec![];
    loop {
      match stream.read_to_end(&mut vector) {
        Ok(_) => {},
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
        Err(err) => {
          panic!("{}", err);
        }
      };
    };
    String::from_utf8(vector).expect("Could not parse request to string")
  };
  println!("\n\tRequest: {raw_request:?}\n");
  let response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
  stream.write_all(response.as_bytes()).unwrap();
}