mod enums;

use std::collections::HashMap;
use std::io::Error;
use std::rc::Rc;
use crate::http::common::enums::Status;

pub type MiddlewareList = Vec<Middleware>;

pub type Middleware = Rc<dyn Fn(&Request) -> Result<(), Error>>;

pub type ApplicationMap = HashMap<String, MethodsMap>;

pub type MethodsMap = HashMap<String, RequestHandler>;

pub type RequestHandler = Box<dyn Fn(&Request) -> Response>;

pub type HeadersMap = HashMap<String, String>;

pub struct Request {
  method: Option<String>,
  route: Option<String>,
  protocol: Option<String>,
  headers: HeadersMap,
  content: Vec<u8>
}

impl Default for Request {
  fn default() -> Self {
    let method = None;
    let route = None;
    let protocol = None;
    let headers = HeadersMap::new();
    let content = Vec::new();
    Self { method, route, protocol, headers, content }
  }
}

impl Request {
  pub fn set_method(&mut self, method: String) {
    self.method = Some(method);
  }

  pub fn set_route(&mut self, route: String) {
    self.route = Some(route);
  }

  pub fn set_protocol(this: &mut Self, protocol: String) {
    this.protocol = Some(protocol);
  }

  pub fn set_headers(&mut self, headers: HeadersMap) {
    self.headers = headers;
  }

  pub fn add_header(&mut self, header_key: String, header_value: String) {
    self.headers.insert(header_key, header_value);
  }

  pub fn set_content(&mut self, content: Vec<u8>) {
    self.content = content
  }

  pub fn push_char_to_content(this: &mut Self, char: u8) {
    this.content.push(char);
  }
}

pub struct Response {
  protocol: Option<String>,
  status_code: usize,
  status: String,
  headers: HeadersMap,
  body: Option<String>
}

impl Response {
  pub fn add_header(&mut self, key: String, value: String) {
    self.headers.insert(key, value);
  }
  pub fn with_body(&mut self, body: String) {
    self.body = Some(body);
  }

  fn new(input: Status) -> Self {
    let protocol = Some(String::from("HTTP/1.1"));
    let headers = HeadersMap::new();
    let body = Some(String::new());
    let (status_code, status) = input.to_readable();
    Self { protocol, status, status_code, headers, body }
  }

  pub fn Ok() -> Self {
    Self::new(Status::Ok)
  }

  pub fn Continue() -> Self {
    Self::new(Status::Continue)
  }

  pub fn Processing() -> Self {
    Self::new(Status::Processing)
  }

  pub fn Created() -> Self {
    Self::new(Status::Created)
  }

  pub fn Accepted() -> Self {
    Self::new(Status::Accepted)
  }

  pub fn NonAuthorativeInformation() -> Self {
    Self::new(Status::NonAuthoritativeInformation)
  }

  pub fn NoContent() -> Self {
    Self::new(Status::NoContent)
  }

  pub fn ResetContent() -> Self {
    Self::new(Status::ResetContent)
  }

  pub fn MultipleChoices() -> Self {
    Self::new(Status::MultipleChoices)
  }

  pub fn MovedPermanently() -> Self {
    Self::new(Status::MovedPermanently)
  }

  pub fn Found() -> Self {
    Self::new(Status::Found)
  }

  pub fn SeeOther() -> Self {
    Self::new(Status::SeeOther)
  }

  pub fn NotModified() -> Self {
    Self::new(Status::NotModified)
  }

  pub fn UseProxy() -> Self {
    Self::new(Status::UseProxy)
  }

  pub fn TemporaryRedirect() -> Self {
    Self::new(Status::TemporaryRedirect)
  }

  pub fn PermanentRedirect() -> Self {
    Self::new(Status::PermanentRedirect)
  }

  pub fn BadRequest() -> Self {
    Self::new(Status::BadRequest)
  }

  pub fn Forbidden() -> Self {
    Self::new(Status::Forbidden)
  }

  pub fn NotFound() -> Self {
    Self::new(Status::NotFound)
  }

  pub fn MethodNotAllowed() -> Self {
    Self::new(Status::MethodNotAllowed)
  }

  pub fn RequestTimeout() -> Self {
    Self::new(Status::RequestTimeout)
  }

  pub fn Conflict() -> Self {
    Self::new(Status::Conflict)
  }

  pub fn Gone() -> Self {
    Self::new(Status::Gone)
  }

  pub fn PayloadTooLarge() -> Self {
    Self::new(Status::PayloadTooLarge)
  }

  pub fn UnsupportedMediaType() -> Self {
    Self::new(Status::UnsupportedMediaType)
  }

  pub fn Locked() -> Self {
    Self::new(Status::Locked)
  }

  pub fn TooManyRequests() -> Self {
    Self::new(Status::TooManyRequests)
  }

  pub fn InternalServerError() -> Self {
    Self::new(Status::InternalServerError)
  }

  pub fn BadGateway() -> Self {
    Self::new(Status::BadGateway)
  }

  pub fn ServiceUnavailable() -> Self {
    Self::new(Status::ServiceUnavailable)
  }

  pub fn GatewayTimeout() -> Self {
    Self::new(Status::GatewayTimeout)
  }
}

/// pub struct _Application_
/// This struct aims to store all routes and methods handlers and all middlewares
/// The middlewares will run in all requests and must receive a Request reference and returns nothing
///
pub struct Application {
  middlewares: MiddlewareList,
  handlers: ApplicationMap
}

impl Default for Application {
  fn default() -> Self {
    let middlewares = MiddlewareList::new();
    let handlers = ApplicationMap::new();
    Self { middlewares, handlers }
  }
}

impl Application {
  pub fn set_middlewares(&mut self, middlewares: MiddlewareList) {
    self.middlewares = middlewares;
  }

  pub fn add_middleware(&mut self, middleware: Middleware) {
    self.middlewares.push(middleware)
  }

  pub fn middlewares(&self) -> MiddlewareList {
    let cloned = self
      .middlewares
      .to_vec();
    cloned
  }

  pub fn set_handlers(this: &mut Self, handlers: ApplicationMap) {
    this.handlers = handlers;
  }

  pub fn add_handler(&mut self, route: String, method: String, handler: RequestHandler) {
    let mut methods = self.get_route_available_handlers(&route);
    methods.insert(method, handler);
    self.set_route_handlers(route, methods);
  }

  fn get_route_available_handlers(&mut self, route: &String) -> MethodsMap {
    match self.handlers.remove(route) {
      None => MethodsMap::new(),
      Some(map) => map,
    }
  }

  fn set_route_handlers(&mut self, route: String, methods_map: MethodsMap) {
    self.handlers.insert(route, methods_map).expect("Unable to append route map");
  }
}