use std::io::Error;
use tokio::net::TcpListener;
use crate::http::common::{Application, Middleware, MiddlewareList};

pub struct Server {
  listener: TcpListener,
  handler: Application
}

impl Server {
  pub async fn bind(address: &str, port: u16) -> Result<Self, Error> {
    let formatted_address = format!("{address}:{port}");
    let listener = TcpListener::bind(formatted_address).await?;
    let handler = Application::default();
    Ok(Self { listener, handler })
  }

  pub fn set_middlewares(&mut self, middleware_list: MiddlewareList) {
    self.handler.set_middlewares(middleware_list)
  }

  pub fn add_middleware(&mut self, middleware: Middleware) {
    self.handler.add_middleware(middleware)
  }


}