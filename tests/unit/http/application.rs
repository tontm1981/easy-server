use easy_server::http::{application::Application, response::Response, request::Request};

use crate::helpers::type_of::type_of;

#[test]
fn new_application_instance() {
    let sut = Application::new();
    let sut_type = type_of(&sut);
    assert!(sut_type == "easy_server::http::application::Application");
}

#[test]
fn assert_application_instance_can_handle_requests() {
    let mut sut = Application::new();
    fn handle_request(_: &Request) -> Option<easy_server::http::response::Response> { 
        Some(
            Response::new(
                String::from("HTTP/1.1"),
                200, 
                "Ok".to_string(), 
                String::from("Fake néri")
            )
        )
    }
    sut.get(
        "/".to_string(), 
        vec![Box::new(handle_request)]
    );
    let request = Request::new(
        "GET".to_string(),
        "/".to_string(), 
        String::from("HTTP/1.1")
    );
    let response = sut.handle_request(request);
    println!("{response:#?}");
    assert!(response.contains("Ok"));
    assert!(response.contains("200"));
    assert!(response.contains("Fake néri"));
}