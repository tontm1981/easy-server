#[cfg(test)]
mod integration {
    use std::io::Error;
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;
    use easy_server::http::server::{
        common::{
            Request,
            traits::HttpServer,
        },
        single::Server,
        std_thread::Server as TServer
    };

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

    #[test]
    fn assert_multiple_get_request_is_ok_using_thread_server() {
        thread::spawn(|| {
            let mut sut = TServer::try_new("127.0.0.1", 6868).unwrap();
            sut.get("/sleep".to_string(), |_: &Request| -> Result<Option<String>, Error> {
                thread::sleep(Duration::from_secs(5));
                Ok(None)
            });
            sut.run().unwrap();
        });

        thread::sleep(Duration::from_millis(100));
        let mut ts = Vec::<JoinHandle<()>>::new();
        for id in 0..5 {
            let th = thread::spawn(move || {
                println!("Doing request #{id}");
                let response = reqwest::blocking::get("http://localhost:6868/sleep").unwrap();
                assert_eq!(204, response.status());
            });
            ts.push(th);
        }

        for t in ts {
            t.join().unwrap();
        }
    }

    #[test]
    fn assert_multiple_get_request_is_ok() {
        thread::spawn(|| {
            let mut sut = create_new_server(2728);
            sut.get("/sleep".to_string(), |_: &Request| -> Result<Option<String>, Error> {
                thread::sleep(Duration::from_secs(5));
                Ok(None)
            });
            sut.run().unwrap();
        });

        thread::sleep(Duration::from_millis(100));
        let mut ts: Vec<JoinHandle<()>> = Vec::new();
        for id in 0..5 {
            let th = thread::spawn(move || {
                println!("Doing request #{id}");
                let response = reqwest::blocking::get("http://localhost:2728/sleep").unwrap();
                assert_eq!(204, response.status());
            });
            ts.push(th);
        }

        for t in ts {
            t.join().unwrap();
        }
    }
}
