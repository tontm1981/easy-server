#[cfg(test)]
mod integration {
    use std::io::Error;
    use std::thread;
    use std::time::Duration;
    use easy_server::http::server::common::Request;
    use easy_server::http::server::single::Server;

    #[test]
    fn assert_server_is_up_and_running() {
        thread::spawn(|| {
            let mut sut = create_new_server(6565);
            sut.run().unwrap();
        });
        thread::sleep(Duration::from_millis(100));
        let response = reqwest::blocking::get("http://localhost:6565/").unwrap();
        assert_eq!(404, response.status());
    }

    fn create_new_server(port: usize) -> Server {
        let sut = Server::try_new("127.0.0.1", port).unwrap();
        sut
    }

    #[test]
    fn assert_valid_get_request_is_ok() {
        thread::spawn(|| {
            let mut sut = create_new_server(5656);
            sut.get("/sleep".to_string(), |_: &Request| -> Result<Option<String>, Error> {
                thread::sleep(Duration::from_secs(5));
                Ok(None)
            });
            sut.run().unwrap();
        });
        thread::sleep(Duration::from_millis(100));
        let response = reqwest::blocking::get("http://localhost:5656/sleep").unwrap();
        assert_eq!(204, response.status());
    }
}