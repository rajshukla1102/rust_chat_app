use std::io::{self, prelude::*};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Could not connect to the server");
    let mut name = String::new();
    println!("input your name: ");
    io::stdin().read_line(&mut name).expect("failed to read from stdin");
    stream.write_all(name.as_bytes()).expect("failed to send name to server");
    loop {
        let mut input = String::new();
        let mut buffer = [0; 1024];
        io::stdin().read_line(&mut input).expect("failed to read from stdin");
        stream.write(input.as_bytes()).expect("Failed to Write to server");

        // if you are echoing something back from server then uncomment below line
        // stream.read(&mut buffer).expect("Failed to read from server"); 
        // print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}