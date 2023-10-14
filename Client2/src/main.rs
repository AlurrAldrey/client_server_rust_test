use std::str;
use std::env;
use std::net::TcpStream;
use std::io::{self,prelude::*,BufReader,Write};

fn main() {
    
    let vec_messages = vec!["C2Paso1\n", "C2Paso2\n", "Sleep\n", "C2Paso3\n"];
    
    for message in vec_messages {
        
        let args: Vec<String> = env::args().collect();
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        
        println!("We are sending: {}", message);
        
        stream.write(message.as_bytes()).expect("failed to write");
        // Add buffering so that the receiver can read messages from the stream
        let mut reader = BufReader::new(&stream);
        
        let request_line = reader.lines().next().unwrap().unwrap();
        println!("read from server:{}", &request_line);
        println!("");
    }

}