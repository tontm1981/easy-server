use super::common::Headers;

pub struct Response {
    code: u16,
    status: String,
    protocol: String,
    content: String,
    headers: Option<Headers>,
}
impl Response {
    pub fn new(protocol: String, code: u16, status: String, content: String) -> Self {
        Self { code, status, content, protocol, headers: None }
    }

    pub fn set_content(&mut self, content: String) {
        if !content.is_empty() {
            self.add_header(String::from("Content-length"), content.len().to_string());
            self.content = content;
        }
    }

    pub fn add_header(&mut self, header_name: String, header_value: String) {
        let mut header = match &self.headers {
            None => Headers::new(),
            Some(h) => h.to_owned(),
        };
        header.insert(header_name, header_value);
        self.headers = Some(header);
    }

    pub fn to_string(&self) -> String {
        let mut response = format!("{} {} {}\r\n\r\n", self.protocol, self.code, self.status);
        match &self.headers {
            None => {},
            Some(h) => {
                h.iter().for_each(|(k,v)| response.push_str(&format!("{}:{}", k, v)));
                response.push_str("\r\n\r\n");
            },
        }
        if !self.content.is_empty() {
            response.push_str(&self.content);
        }
        response
    }
}
