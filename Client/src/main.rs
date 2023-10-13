use std::str;
use std::env;
use std::net::TcpStream;
use std::io::{self,prelude::*,BufReader,Write};

fn main() {
    
    let vec_messages = vec!["Sleep\n", "__\n", "hello\n", "Sleep\n"];
    
    for message in vec_messages {
        // connect
        // Struct used to start requests to the server.
        // Check TcpStream Connection to the server
        let args: Vec<String> = env::args().collect();
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        println!("We are sending: {}", message);
        stream.write(message.as_bytes()).expect("failed to write");
        // Add buffering so that the receiver can read messages from the stream
        let mut reader = BufReader::new(&stream);
        // Check if this input message values are u8
        let mut buffer: Vec<u8> = Vec::new();
        // Read input information
        // reader.read_until(b'\n',&mut buffer).unwrap();
       
        println!("read from server:{}",str::from_utf8(&buffer).unwrap());
        println!("");
    }

}