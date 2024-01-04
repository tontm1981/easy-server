use std::{thread, time::Duration};
use easy_server::http::server::SingleServer;

#[test]
fn assert_single_server_is_runing() {
    thread::spawn(|| {
        SingleServer::try_new("127.0.0.1", 4545)
            .unwrap()
            .run()
            .unwrap();
    });
    thread::sleep(Duration::from_millis(100));
    let response = reqwest::blocking::get("http://localhost:4545").expect("Failed to request single server");
    assert_eq!(response.status(), 200);
}