use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

const HOST: &str = "127.0.0.1";
const PORT: &str = "7878";
const WEB_DIR: &str = "www";

fn main() {
    println!("Server is starting ...");
    // bind host and PORT
    let endpoint = format!("{}:{}", HOST, PORT);
    let listener: TcpListener = TcpListener::bind(&endpoint).unwrap();
    println!("Server running on: http://{}", &endpoint);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(stream);
            },
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    
    stream.read(&mut buffer).unwrap();
    // println!(
    //     "Request: {}",
    //     String::from_utf8_lossy(&buffer[..])
    // );
    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();

        let response: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        println!("error 404");
    }

    
}
