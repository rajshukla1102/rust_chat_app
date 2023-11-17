use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let byte_count = stream.read(&mut buffer).unwrap();
    let client_name = String::from_utf8_lossy(&buffer[..byte_count]).to_string();

    println!("{} has connected", client_name);

    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("{} disconnected", client_name);
                    break;
                }
                let msg = format!("{} says: {}",
                    client_name,
                    String::from_utf8_lossy(&buffer[..bytes_read]));
                    println!("{}", msg);
                    // stream.write_all(&buffer[..bytes_read]).expect("couldnt reply back to client"); 
                    // it is not necessary to echo back to the client so commented above line
            }
            Err(e) => {
                eprintln!("Problem handling client: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let lis: TcpListener = TcpListener::bind("127.0.0.1:7878").expect("couldnt bind");
    for stream in lis.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprint!("failed to accept connection: {}", e);
            }
        };
    }
}