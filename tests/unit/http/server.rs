use easy_server::http::server::SingleServer;
use crate::helpers::type_of::type_of;


#[test]
fn create_new_single_server_instance() {
    let sut = SingleServer::try_new("127.0.0.1", 8585).expect("Couldn't create the server instance");
    let sut_type = type_of(&sut);
    assert_eq!(sut_type, "easy_server::http::server::SingleServer");
}