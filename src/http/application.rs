use std::collections::HashMap;

use super::{request::Request, response::Response};

type FunctionList = Vec<
    Box<
        dyn Fn(&Request) -> Option<Response>
    >
>;

type RouteMap = HashMap<
    String, 
    FunctionList
>;

type ApplicationMap = HashMap<
    String, 
    RouteMap
>;
pub struct Application (
    ApplicationMap
);
impl Application {
    pub fn new() -> Self {
        Self(ApplicationMap::new())
    }

    pub fn connect(&mut self, route: String, f: FunctionList) {
        self.route("CONNECT", route, f);
    }

    pub fn delete(&mut self, route: String, f: FunctionList) {
        self.route("DELETE", route, f);
    }

    pub fn get(&mut self, route: String, f: FunctionList) {
        self.route("GET", route, f);
    }

    pub fn head(&mut self, route: String, f: FunctionList) {
        self.route("HEAD", route, f);
    }

    pub fn options(&mut self, route: String, f: FunctionList) {
        self.route("OPTIONS", route, f);
    }

    pub fn patch(&mut self, route: String, f: FunctionList) {
        self.route("PATCH", route, f);
    }

    pub fn post(&mut self, route: String, f: FunctionList) {
        self.route("POST", route, f)
    }

    pub fn put(&mut self, route: String, f: FunctionList) {
        self.route("PUT", route, f);
    }

    pub fn trace(&mut self, route: String, f: FunctionList) {
        self.route("TRACE", route, f);
    }

    fn route(&mut self, verb: &str, route: String, funcs: FunctionList) {
        let mut map = self.0.remove(verb).unwrap();
        map.insert(route, funcs);
        self.0.insert(verb.to_string(), map);
    }

    pub fn handle_request(&self, request: Request) -> String {
        let mut response: Response = self.build_not_found(&request);
        match self.0.get(request.method()) {
            None => {
                response = self.build_not_found(&request);
            },
            Some(routes) => {
                match routes.get(request.route()) {
                    None => {
                        response = self.build_not_found(&request);
                    },
                    Some(functions) => {
                        for function in functions {
                            match function(&request) {
                                None => {},
                                Some(r) => {
                                    response = r;
                                },
                            }
                        }
                    }
                }
            }
        };
        response.to_string()
    }

    fn build_not_found(&self, request: &Request) -> Response {
        Response::new(
            request.protocol().to_string(), 
            404, 
            String::from("Not Found"),
            "".to_string()
        )
    }
}