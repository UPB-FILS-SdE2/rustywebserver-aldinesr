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
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let root_folder = Arc::new(PathBuf::from(&args[2]));

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("Server running on port {}", port);

    loop {
        let (socket, _) = listener.accept().await?;
        let root_folder = Arc::clone(&root_folder);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, root_folder).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream, root_folder: Arc<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let bytes_read = socket.read(&mut buffer).await?;

    if bytes_read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let (method, path, headers, body) = parse_request(&request);

    match method.as_str() {
        "GET" => handle_get(socket, root_folder, &path).await?,
        "POST" => handle_post(socket, root_folder, &path, headers, body).await?,
        _ => {
            let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
            socket.write_all(response.as_bytes()).await?;
        }
    }

    Ok(())
}

fn parse_request(request: &str) -> (String, String, HashMap<String, String>, String) {
    let mut lines = request.lines();
    let first_line = lines.next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();

    let mut headers = HashMap::new();
    let mut body = String::new();
    let mut in_body = false;

    for line in lines {
        if line.is_empty() {
            in_body = true;
            continue;
        }

        if in_body {
            body.push_str(line);
        } else {
            let mut header_parts = line.splitn(2, ':');
            let key = header_parts.next().unwrap_or("").trim().to_string();
            let value = header_parts.next().unwrap_or("").trim().to_string();
            headers.insert(key, value);
        }
    }

    (method, path, headers, body)
}

async fn handle_get(mut socket: TcpStream, root_folder: Arc<PathBuf>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = root_folder.join(&path[1..]); // Skip the initial '/'

    if !file_path.starts_with(root_folder.as_path()) {
        let response = "HTTP/1.1 403 Forbidden\r\n\r\n";
        socket.write_all(response.as_bytes()).await?;
        return Ok(());
    }

    match fs::read(&file_path).await {
        Ok(contents) => {
            let content_type = determine_content_type(&file_path);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                content_type,
                contents.len()
            );
            socket.write_all(response.as_bytes()).await?;
            socket.write_all(&contents).await?;
        }
        Err(_) => {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            socket.write_all(response.as_bytes()).await?;
        }
    }

    Ok(())
}
async fn handle_post(
    mut socket: TcpStream,
    root_folder: Arc<PathBuf>,
    path: &str,
    headers: HashMap<String, String>,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let script_path = root_folder.join(&path[1..]);

    if !script_path.starts_with(root_folder.as_path()) || !script_path.exists() {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        socket.write_all(response.as_bytes()).await?;
        return Ok(());
    }

    let mut command = Command::new(&script_path);
    command.envs(headers).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = command.spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(body.as_bytes()).await?;
    }

    let output = child.wait_with_output().await?;

    let status_code = if output.status.success() {
        "200 OK"
    } else {
        "500 Internal Server Error"
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        output.stdout.len(),
        String::from_utf8_lossy(&output.stdout)
    );

    socket.write_all(response.as_bytes()).await?;

    Ok(())
}


fn determine_content_type(file_path: &Path) -> &'static str {
    match file_path.extension().and_then(|ext| ext.to_str()) {
        Some("txt") => "text/plain; charset=utf-8",
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("zip") => "application/zip",
        _ => "application/octet-stream",
    }
}