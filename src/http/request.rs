pub struct Request {
    method: String,
    route: String,
    protocol: String,
}
impl Request {
    pub fn new(method: String, route: String, protocol: String) -> Self {
        Self { method, route, protocol }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn route(&self) -> &str {
        &self.route
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }
}
