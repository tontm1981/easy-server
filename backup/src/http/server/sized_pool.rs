use std::{net::TcpListener, thread};

pub struct Server {
    listener: TcpListener,
    pool: thread::JoinHandle<()>,
}

impl Server {
    pub fn new(listener: TcpListener, pool: thread::JoinHandle<()>) -> Self { Self { listener, pool } }
}
