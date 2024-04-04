#[cfg(test)]
mod integration {
    mod single {
        use std::thread;
        use std::time::Duration;
        use easy::http::single::Server;

        #[test]
        fn assert_server_is_running() {
            thread::spawn(|| {
                Server::bind("127.0.0.1", 8989)
                    .expect("Unable to initiate the server")
                    .run();
            });
            thread::sleep(Duration::from_millis(100));
            let output = reqwest::blocking::get("http://127.0.0.1:8989/").unwrap();
            println!("{output:#?}");
            assert_eq!(output.status(), 200);
        }
    }
}