use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub struct Request {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: String,
}

pub struct Response {
    status_line: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    fn new(status_line: &str, body: &str) -> Self {
        let headers = vec![
            ("Content-Length".to_string(), body.len().to_string()),
            ("Content-Type".to_string(), "text/html".to_string()),
        ];
        Response {
            status_line: status_line.to_string(),
            headers,
            body: body.to_string(),
        }
    }
    
    fn to_string(&self) -> String {
        let mut response = format!("{}\r\n", self.status_line);
        for (k, v) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", k, v));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}

pub type Handler = fn(Request) -> Response;

pub struct Router {
    routes: Vec<(String, Handler)>,
}

impl Router {
    fn new() -> Self {
        Router { routes: Vec::new() }
    }
    
    fn add_route(&mut self, path: &str, handler: Handler) {
        self.routes.push((path.to_string(), handler));
    }
    
    // Jeśli nie znajdzie pasującej, zwraca 404.
    fn route(&self, req: Request) -> Response {
        for (route, handler) in &self.routes {
            if route == &req.path {
                return handler(req);
            }
        }
        Response::new("HTTP/1.1 404 NOT FOUND", "<h1>404 Not Found</h1>")
    }
}

pub fn parse_request(buffer: &[u8]) -> Request {
    let request_str = String::from_utf8_lossy(buffer);
    let mut lines = request_str.lines();
    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();
    
    let mut headers = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.push((key.to_string(), value.to_string()));
        }
    }
    
    let body = lines.collect::<Vec<&str>>().join("\n");
    
    Request { method, path, headers, body }
}