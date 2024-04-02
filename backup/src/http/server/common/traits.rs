use crate::http::server::common::types::ApplicationHandler;

pub trait HttpServer {
    fn connect(&mut self, route: String, function: ApplicationHandler);
    fn delete(&mut self, route: String, function: ApplicationHandler);
    fn get(&mut self, route: String, function: ApplicationHandler);
    fn head(&mut self, route: String, function: ApplicationHandler);
    fn options(&mut self, route: String, function: ApplicationHandler);
    fn patch(&mut self, route: String, function: ApplicationHandler);
    fn post(&mut self, route: String, function: ApplicationHandler);
    fn put(&mut self, route: String, function: ApplicationHandler);
    fn trace(&mut self, route: String, function: ApplicationHandler);
}