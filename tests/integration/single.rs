use std::thread;
use std::time::Duration;

#[test]
fn assert_server_is_running() {
  thread::spawn(|| {
    Server::try_new("127.0.0.1", 8989)
      .expect("Unable to initiate the server")
      .serve()
      .unwrap();
  });
  thread::sleep(Duration::from_millis(75));
  let output = reqwest::blocking::get("http://127.0.0.1:8989/").expect("Unable to perform a GET request");
  assert_eq!(output.status(), 200);
}