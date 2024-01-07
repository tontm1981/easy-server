use easy_server::http::server::SingleServer;
use crate::helpers::type_of::type_of;

#[test]
fn create_new_single_server_instance() {
    let sut = SingleServer::try_new("127.0.0.1", 8585).expect("Couldn't create the server instance");
    let sut_type = type_of(&sut);
    assert_eq!(sut_type, "easy_server::http::server::SingleServer");
}

#[test]
fn parse_request_string() {
    let sut = SingleServer::try_new("127.0.0.1", 2222).expect("Couldn't create the server instance");
    let request_string = "GET / HTTP/1.1\r\naccept: */*\r\nhost: localhost:7878\r\n\r\n".to_string();
    let result = sut.pub_parse_request_string(request_string);
    println!("{result:#?}");
    let typeof_result = type_of(&result);
    assert_eq!(typeof_result, "alloc::vec::Vec<alloc::string::String>");
}
