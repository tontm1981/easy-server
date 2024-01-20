#[cfg(test)]
mod integration {
    use std::thread;
    use std::time::Duration;
    use easy_server::Server;

    #[test]
    fn assert_server_is_up_and_running() {
        thread::spawn(|| {
            let sut = create_new_server();
            sut.run().unwrap();
        });
        thread::sleep(Duration::from_millis(100));
        let response = reqwest::blocking::get("http://localhost:7878/").unwrap();
        assert_eq!(200, response.status());
    }

    fn create_new_server() -> Server {
        let sut = Server::try_new("127.0.0.1", 7878).unwrap();
        sut
    }
}