// use std::net::TcpListener;
use std::env;
// use std::fs;
// use tokio::stream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpListener;

use std::sync::Arc;
use std::error::Error;

use tokio::net::{TcpListener, TcpStream};
use tokio::fs;
use tokio::process::Command;
use std::process::Stdio;

use std::collections::HashMap;

use std::path::{Path, PathBuf};
use tokio::fs::File;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <PORT> <ROOT_FOLDER>", args[0]);
        std::process::exit(1);
    }

    let port = &args[1];
    let root_folder = &args[2];

    println!("Root folder: {}", fs::canonicalize(root_folder).await?.display());
    println!("Server listening on 0.0.0.0:{}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    let root = Arc::new(root_folder.to_string());

    loop {
        let (stream, _) = listener.accept().await?;
        let root = Arc::clone(&root);
        tokio::spawn(async move {
            if let Err(e) = connections(stream, root).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn connections(mut stream: TcpStream, root: Arc<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 8192];
    let bytes_read = stream.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    
    let (request_line, headers, body) = parse_request(&request);
    let (method, path) = parse_request_line(&request_line);
    
    let client_ip = stream.peer_addr()?.ip().to_string();

    match method {
        "GET" => handle_get(&mut stream, &root, path, &client_ip).await?,
        "POST" => handle_post(&mut stream, &root, path, &client_ip, body).await?,
        _ => send_response(&mut stream, 405, "Method Not Allowed", "text/html; charset=utf-8", "<html>405 Method Not Allowed</html>").await?,
    }

    Ok(())
}


fn parse_request(request: &str) -> (String, HashMap<String, String>, String) {
    let mut parts = request.split("\r\n\r\n");
    let header_part = parts.next().unwrap_or("");
    let body = parts.next().unwrap_or("").to_string();

    let mut headers = HashMap::new();
    let mut lines = header_part.lines();
    let request_line = lines.next().unwrap_or("").to_string();

    for line in lines {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    (request_line, headers, body)
}

fn parse_request_line(request_line: &str) -> (&str, &str) {
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");
    (method, path)
}

async fn handle_get(stream: &mut TcpStream, root: &str, path: &str, client_ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let requested_path = Path::new(root).join(path.trim_start_matches('/'));

    if requested_path.is_file() {
        let content = fs::read(&requested_path).await?;
        let content_type = get_content_type(&requested_path);
        send_response(stream, 200, "OK", &content_type, &String::from_utf8_lossy(&content)).await?;
    } else if requested_path.is_dir() {
        send_response(stream, 200, "OK", "text/html; charset=utf-8", "<html><h1>Directory Listing</h1></html>").await?;
    } else {
        send_response(stream, 404, "Not Found", "text/html; charset=utf-8", "<html>404 Not Found</html>").await?;
    }

    Ok(())
}



async fn handle_post(stream: &mut TcpStream, root: &str, path: &str, client_ip: &str, body: String) -> Result<(), Box<dyn std::error::Error>> {
    send_response(stream, 405, "Method Not Allowed", "text/html; charset=utf-8", "<html>405 Method Not Allowed</html>").await?;
    Ok(())
}

async fn send_response(stream: &mut TcpStream, status_code: u32, status: &str, content_type: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_code, status, content_type, body.len(), body
    );
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
fn get_content_type(path: &Path) -> String {
    match path.extension().and_then(std::ffi::OsStr::to_str) {
        Some("html") => "text/html; charset=utf-8".to_string(),
        Some("css") => "text/css; charset=utf-8".to_string(),
        Some("js") => "application/javascript".to_string(),
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("png") => "image/png".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}
