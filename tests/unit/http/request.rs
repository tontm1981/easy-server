use crate::helpers::type_of::type_of;

fn create_new_request_object() {
    let sut = Request::new("GET", "/", "HTTP1.1");
    let sut_type = type_of(&sut);
    assert_eq!(sut_type, "easy_server::http::request::Request");
}