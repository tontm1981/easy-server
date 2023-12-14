use easy_server::http::request::Request;

use crate::helpers::type_of::type_of;

#[test]
fn create_new_request_object() {
    let sut = Request::new("GET".to_string(), String::from("/"), "HTTP1.1".to_string());
    let sut_type = type_of(&sut);
    assert_eq!(sut_type, "easy_server::http::request::Request");
}