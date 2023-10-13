use client_server_rust_test::ThreadPool;
use std::thread;
use std::time::Duration;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("We got: {}", request_line);
    if request_line == "Sleep" {
        let status_line = "Just woke, what a nap of 10 seconds :D\n";
        let response = format!(
            "{status_line}\r\n"
        );
        
        thread::sleep(Duration::new(2, 0));
        stream.write_all(response.as_bytes()).unwrap();

    } else {        
        
        stream.write_all(request_line.as_bytes()).unwrap();
    }
}