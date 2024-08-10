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


async fn handle_request(mut socket: tokio::net::TcpStream, root_folder: PathBuf) {
    let mut buffer = [0; 1024];
    let _ = socket.read(&mut buffer).await;
    let request = String::from_utf8_lossy(&buffer[..]);
    let path = if let Some(line) = request.lines().next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "GET" {
            parts[1]
        } else {
            "/"
        }
        } else {
          "/"
        };
        let path = if path == "/" { "index.html" } else { path.trim_start_matches('/') };
        let full_path = root_folder.join(path);

    let response = if full_path.exists() && full_path.is_file() {
        match File::open(full_path).await {
            Ok(mut file) => {
                let mut contents = Vec::new();
                if let Err(_) = file.read_to_end(&mut contents).await {
                    "HTTP/1.1 500 Internal Server Error\r\n\r\nError reading file".to_string()
                } else {
                    let header = "HTTP/1.1 200 OK\r\n\r\n";
                    format!("{}{}", header, String::from_utf8_lossy(&contents))
                }
            }
            Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\nError opening file".to_string(),
        }
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nFile not found".to_string()
    };

    let _ = socket.write_all(response.as_bytes()).await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Test ");

    let args: Vec<String> = env::args().collect();

    if args.len()!=3{
        eprintln!("Usage:{} <PORT><ROOT_FOLDER>",args[0]);
        std::process::exit(1);
    }
    
    let port = &args[1];
    // let root_folder = args[2].clone();
    let root_folder = PathBuf::from(&args[2]);
    // println!("Test Port {} ",args[1]);
    // println!("Test root folder {} ",args[2]);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    let addr=format!("0.0.0.0:{}",port);
  
    loop{
        let (socket, _) = listener.accept().await?;
        let root_folder = root_folder.clone();

     tokio::spawn(async move{
        handle_request(socket, root_folder).await;
     });
    }
}
