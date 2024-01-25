pub mod types;
pub mod enums;

use std::collections::HashMap;
use crate::common::enums::HttpStatuses;
use crate::common::types::{ApplicationHandler, ApplicationMap, MiddlewareFunctionsVec, RouteMap};

type HeaderMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Request {
    method: String,
    uri: String,
    protocol: String,
    headers: HeaderMap,
    body: Option<String>
}

impl Request {
    pub fn from_string(request: String) -> Self {
        let body = None;
        let mut lines: Vec<String> = request
            .split("\r\n")
            .map(|s| s.to_string())
            .collect();
        let request_settings = lines
            .remove(0)
            .splitn(3, ' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut headers = HeaderMap::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            let (key, value) = line
                .split_once(':')
                .unwrap();
            headers.insert(key.to_string(), value.to_string());
        }

        Self{
            body,
            headers,
            method: request_settings[0].to_owned(),
            uri: request_settings[1].to_owned(),
            protocol: request_settings[2].to_owned()
        }
    }

    pub fn method(&self) -> String {
        self.method.clone()
    }

    pub fn uri(&self) -> String {
        self.uri.clone()
    }

    pub fn protocol(&self) -> String {
        self.protocol.clone()
    }

    pub fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    pub fn raw_body(&self) -> Option<String> {
        self.body.clone()
    }
}

pub struct Response {
    protocol: String,
    status: usize,
    description: String,
    headers: HeaderMap,
    content: String,
}
impl Response {
    pub fn new(protocol: String, status: usize, description: String) -> Self {
        let headers = HeaderMap::new();
        let content = String::new();
        Self {
            protocol,
            status,
            description,
            headers,
            content
        }
    }

    pub fn build_from_status(protocol: String, status: HttpStatuses) -> Self {
        let (status, description) = status.to_response();
        Self::new(protocol, status, description)
    }

    pub fn set_headers(&mut self, headers: HeaderMap) {
        self.headers = headers;
    }

    pub fn update_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn set_content(&mut self, content: String) {
        let length = content.len();
        self.update_header("Content-Length".to_string(), length.to_string());
        self.content = content;
    }

    pub fn to_string(&self) -> String {
        let mut response = format!("{} {} {}\r\n", self.protocol, self.status, self.description);

        if self.headers.len() > 0 {
            let headers = self.headers.clone();
            for (key, value) in headers {
                let header = format!("{key}:{value}\r\n");
                response.push_str(header.as_str());
            }
        }

        response.push_str("\r\n");

        if self.content.len() > 0 {
            response.push_str(self.content.as_str());
            response.push_str("\r\n\r\n");
        }
        
        response
    }

}

#[derive(Clone)]
pub struct Application(ApplicationMap, MiddlewareFunctionsVec);
impl Application {
    pub fn new() -> Self {
        let app_map = ApplicationMap::new();
        let middlewares_map = MiddlewareFunctionsVec::new();
        Self(app_map, middlewares_map)
    }

    fn route(&mut self, method: String, route: String, function: ApplicationHandler) {
        let mut route_map = self.get_method_mappings(&method);
        route_map.insert(route, function);
        self.0.insert(method, route_map);
    }

    fn get_method_mappings(&mut self, method: &String) -> RouteMap {
        match self.0.remove(method) {
            None => RouteMap::new(),
            Some(map) => map,
        }
    }

    pub fn get_handler(&mut self, method: String, route: String) -> Option<ApplicationHandler> {
        match self. 0.get(&method) {
            Some(_) => {
                let mut route_map = self.get_method_mappings(&method);
                let function = route_map.remove(&route);
                route_map.insert(route, function.unwrap());
                self.0.insert(method, route_map);
                function
            },
            None =>  None,
        }
    }

    pub fn connect(&mut self, route: String, function: ApplicationHandler) {
        self.route(String::from("CONNECT"), route, function);
    }

    pub fn delete(&mut self, route: String, function: ApplicationHandler) {
        self.route(String::from("DELETE"), route, function);
    }

    pub fn get(&mut self, route: String, function: ApplicationHandler) {
        self.route(String::from("GET"), route, function);
    }

    pub fn head(&mut self, route: String, function: ApplicationHandler) {
        self.route(String::from("HEAD"), route, function);
    }

    pub fn options(&mut self, route: String, function: ApplicationHandler) {
        self.route(String::from("OPTIONS"), route, function);
    }

    pub fn patch(&mut self, route: String, function: ApplicationHandler) {
        self.route(String::from("PATCH"), route, function);
    }

    pub fn post(&mut self, route: String, function: ApplicationHandler) {
        self.route(String::from("POST"), route, function);
    }

    pub fn put(&mut self, route: String, function: ApplicationHandler) {
        self.route(String::from("PUT"), route, function);
    }

    pub fn trace(&mut self, route: String, function: ApplicationHandler) {
        self.route(String::from("TRACE"), route, function);
    }
}