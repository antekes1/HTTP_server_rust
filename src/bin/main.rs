use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

use http_server::ThreadPool;
use http_server::{Router, Response, Request, parse_request};

const HOST: &str = "127.0.0.1";
const PORT: &str = "7878";

fn hello_handler(_req: Request) -> Response {
    Response::new("HTTP/1.1 200 OK", "<h1>Hello, world!</h1>")
}

fn main() {
    println!("Server is starting ...");
    // bind host and PORT
    let endpoint = format!("{}:{}", HOST, PORT);
    let listener: TcpListener = TcpListener::bind(&endpoint).unwrap();
    println!("Server running on: http://{}", &endpoint);

    let pool = ThreadPool::new(4);

    let mut router = Router::new();
    router.add_route("/hello", hello_handler);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                pool.execute( || {
                    handle_connection(stream);
                })
            },
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer: [u8; 1024] = [0; 1024];
    
//     stream.read(&mut buffer).unwrap();
//     // println!(
//     //     "Request: {}",
//     //     String::from_utf8_lossy(&buffer[..])
//     // );
//     let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";

//     let (status_line, filename) = 
//         if buffer.starts_with(get) {
//             ("HTTP/1.1 200 OK", "index.html")
//         } else {
//             ("HTTP/1.1 404 NOT FOUND", "error.html")
//         };
//     let contents = fs::read_to_string(filename).unwrap();

//     let response: String = format!(
//         "{}\r\nContent-Length: {}\r\n\r\n{}",
//         status_line,
//         contents.len(),
//         contents
//     );
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
    

    
// }

fn handle_connection(mut stream: TcpStream, router: &Router) {
    let mut buffer = [0; 1024];
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            return;
        }
        let req = parse_request(&buffer[..bytes_read]);
        let response = router.route(req);
        let response_str = response.to_string();
        stream.write(response_str.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}