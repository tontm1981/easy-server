use std::{thread, time::Duration};
use easy_server::http::{server::SingleServer, request::Request, response::Response};

#[test]
fn assert_single_server_is_runing() {
    thread::spawn(|| {
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
        let mut server = SingleServer::try_new("127.0.0.1", 4545)
            .unwrap();
        server.get("/".to_string(), vec![Box::new(handle_request)]);
        server.run().unwrap();
    });
    thread::sleep(Duration::from_millis(100));
    let response = reqwest::blocking::get("http://localhost:4545").expect("Failed to request single server");
    assert_eq!(response.status(), 200);
}