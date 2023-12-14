use std::{thread, time::Duration};
use easy_server::run;

#[test]
fn assert_server_is_running() {
    println!("Test began");
    thread::spawn(|| {
        run();
    });
    println!("Started server, but is it running?");
    thread::sleep(Duration::from_millis(100));
    let response = reqwest::blocking::get("http://localhost:7878/").expect("Request failed");
    println!("{:#?}", response);
    assert_eq!(response.status(), 200);
}