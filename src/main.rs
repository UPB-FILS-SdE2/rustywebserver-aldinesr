use tokio::net::{TcpListener, TcpStream};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port = &args[1];
    let root_folder = &args[2];

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let root = root_folder.clone();
        tokio::spawn(async move {
            handle_connection(stream, root).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, root: String) {
    let mut buffer = [0; 8192];
    let size = stream.read(&mut buffer).await.unwrap();
    let request = String::from_utf8_lossy(&buffer[..size]);

    let (method, path) = process_request_line(&request);
    match method {
        "GET" => handle_get(&mut stream, &root, &path).await,
        _ => send_response(&mut stream, 405, "Method Not Allowed", "text/html; charset=utf-8", "<html>405 Method Not Allowed</html>").await,
    }
}

fn process_request_line(request: &str) -> (&str, &str) {
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    (method, path)
}

async fn handle_get(stream: &mut TcpStream, root: &str, path: &str) {
    let requested_path = PathBuf::from(root).join(path.trim_start_matches('/'));
    let content = fs::read(requested_path).await.unwrap();
    send_binary_response(stream, 200, "OK", "application/octet-stream", &content).await;
}

async fn send_binary_response(stream: &mut TcpStream, status_code: u32, status: &str, content_type: &str, content: &[u8]) {
    let headers = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        status_code, status, content_type, content.len()
    );
    stream.write_all(headers.as_bytes()).await.unwrap();
    stream.write_all(content).await.unwrap();
}

async fn send_response(stream: &mut TcpStream, status_code: u32, status: &str, content_type: &str, message: &str) {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_code, status, content_type, message.len(), message
    );
    stream.write_all(response.as_bytes()).await.unwrap();
}
