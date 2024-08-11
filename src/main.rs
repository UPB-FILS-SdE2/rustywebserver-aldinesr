// use std::net::TcpListener;
use std::env;
// use std::fs;
// use tokio::stream;
use tokio::io::{AsyncReadExt};
// use tokio::net::TcpListener;
use tokio::io::{self, AsyncWriteExt};
use std::sync::Arc;
use std::error::Error;
use std::io::Error as IoError;
use tokio::net::{TcpListener, TcpStream};
use tokio::fs;
use tokio::process::Command;
// use std::process::Stdio;
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
            if let Err(e) = handle_connection(stream, root).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream, root: Arc<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 8192];
    let size = stream.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..size]);
    

    




    Ok(())
}


fn parse_request(request: &str) -> (String, HashMap<String, String>, String) {
    let mut parts = request.splitn(2, "\r\n\r\n");
    let header_part = parts.next().unwrap_or("");
    let message = parts.next().unwrap_or("").to_string();

    let mut headers = header_part.lines();
    let request_line = headers.next().unwrap_or("").to_string();

    let mut header_map = HashMap::new();

  
    for header in headers {
        if let Some((key, value)) = header.split_once(": ") {
            header_map.insert(key.to_string(), value.to_string());
        }
    }

    (request_line, header_map, message)
}

//possible to modify it
fn process_request_line(request_line: &str) -> (&str, &str, &str) {
    let (method, path, version) = match request_line.split_whitespace().collect::<Vec<_>>().as_slice() {
        [method, path, version, ..] => (*method, *path, *version),
        [method, path] => (*method, *path, ""),
        [method] => (*method, "", ""),
        [] => ("", "", ""),
    };

    (method, path, version)
}


//posible to modify it
// async fn handle_get(
//     stream: &mut TcpStream,
//     root: &str,
//     path: &str,
//     client_ip: &str,
//         ) -> Result<(), Box<dyn std::error::Error>> {
//     let requested_path = PathBuf::from(root).join(path.trim_start_matches('/'));

//     // let normalized_requested_path = if let Ok(p) = fs::canonicalize(&requested_path).await {
//     //     p
//     // } 
//     // else {
//     //     // return send_error_response(stream, client_ip, path, 404, "Not Found").await;
//     // };

   
//     let normalized_root_path = fs::canonicalize(root).await?;

//     if !normalized_requested_path.starts_with(&normalized_root_path) {
//         // return send_error_response(stream, client_ip, path, 403, "Forbidden").await;
//     }

//     match fs::metadata(&normalized_requested_path).await {
//         Ok(metadata) => {
//             if metadata.is_dir() {
//                 handle_directory_listing(stream, &normalized_requested_path, path, client_ip).await?;
//             } else if metadata.is_file() {
//                 handle_file_response(stream, &normalized_requested_path, client_ip, path).await?;
//             } else {
//                 // send_error_response(stream, client_ip, path, 404, "Not Found").await?;
//             }
//         }
//         Err(e) => {
//             eprintln!("Error getting metadata: {:?}", e);
//             // send_error_response(stream, client_ip, path, 404, "Not Found").await?;
//         }
//     }

//     Ok(())
// }

async fn handle_get(stream: &mut TcpStream, root: &str, path: &str, client_ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root_path = PathBuf::from(root);
    let requested_path = root_path.join(path.trim_start_matches('/'));
    
    let normalized_requested_path = match fs::canonicalize(&requested_path).await {
        Ok(p) => p,
        Err(_) => {
            log_request("GET", client_ip, path, 404, "Not Found");
            send_response(stream, 404, "Not Found", "text/html; charset=utf-8", "<html>404 Not Found</html>").await?;
            return Ok(());
        }
    };
    
    let normalized_root_path = fs::canonicalize(&root_path).await?;

    if !normalized_requested_path.starts_with(&normalized_root_path) {
        log_request("GET", client_ip, path, 403, "Forbidden");
        send_response(stream, 403, "Forbidden", "text/html; charset=utf-8", "<html>403 Forbidden</html>").await?;
        return Ok(());
    }

    match fs::metadata(&normalized_requested_path).await {
        Ok(metadata) => {
            if metadata.is_dir() {
                handle_directory_listing(stream, &normalized_requested_path, path, client_ip).await?;
            } else if metadata.is_file() {
                match fs::read(&normalized_requested_path).await {
                    Ok(content) => {
                        let content_type = get_content_type(&normalized_requested_path);
                        log_request("GET", client_ip, path, 200, "OK");
                        send_binary_response(stream, 200, "OK", &content_type, &content).await?;
                    },
                    Err(e) => {
                        eprintln!("Error reading file: {:?}", e);
                        log_request("GET",client_ip, path, 403, "Forbidden");
                        send_response(stream, 403, "Forbidden", "text/html; charset=utf-8", "<html>403 Forbidden</html>").await?;
                    }
                }
            } else {
                log_request("GET",client_ip, path, 404, "Not Found");
                send_response(stream, 404, "Not Found", "text/html; charset=utf-8", "<html>404 Not Found</html>").await?;
            }
        },
        Err(e) => {
            eprintln!("Error getting metadata: {:?}", e);
            log_request("GET",client_ip, path, 404, "Not Found");
            send_response(stream, 404, "Not Found", "text/html; charset=utf-8", "<html>404 Not Found</html>").await?;
        }
    }

    Ok(())
}

//to modify or remove and add it in the handle get
// async fn send_error_response(
//     stream: &mut TcpStream,
//     client_ip: &str,
//     path: &str,
//     status_code: u16,
//     status_message: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     log_request("GET", client_ip, path, status_code, status_message);
//     send_response(
//         stream,
//         status_code,
//         status_message,
//         "text/html; charset=utf-8",
//         &format!("<html>{} {}</html>", status_code, status_message),
//     )
//     .await?;
//     Ok(())
// }

//to modify or remove and add it in the handle get
async fn handle_file_response(
    stream: &mut TcpStream,
    path: &PathBuf,
    client_ip: &str,
    request_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match fs::read(path).await {
        Ok(content) => {
            let content_type = get_content_type(path);
            log_request("GET", client_ip, request_path, 200, "OK");
            send_binary_response(stream, 200, "OK", &content_type, &content).await?;
        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
            // send_error_response(stream, client_ip, request_path, 403, "Forbidden").await?;
        }
    }
    Ok(())
}



async fn send_binary_response(
    stream: &mut TcpStream,
    status_code: u32,
    status: &str,
    content_type: &str,
    content: &[u8]
) -> Result<(), Box<dyn std::error::Error>> {
    let headers = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        status_code, status, content_type, content.len()
    );

    let mut response = Vec::with_capacity(headers.len() + content.len());
    response.extend_from_slice(headers.as_bytes());
    response.extend_from_slice(content);

    stream.write_all(&response).await?;
    Ok(())
}

async fn handle_directory_listing(
    stream: &mut TcpStream,
    full_path: &Path,
    display_path: &str,
    client_ip: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut html = String::with_capacity(1024); 
    html.push_str("<html><h1>Directory Listing</h1><ul>");
    html.push_str("<li><a href=\"..\">..</a></li>");

    let mut entries = fs::read_dir(full_path).await?;
    let mut entry_items = Vec::new(); 

    while let Some(entry) = entries.next_entry().await? {
        if let Some(name) = entry.file_name().to_str() {
            entry_items.push(name.to_string());
        }
    }

  
    for name in entry_items {
        html.push_str(&format!("<li><a href=\"{}\">{}</a></li>", name, name));
    }

    html.push_str("</ul></html>");

  
    log_request("GET", client_ip, display_path, 200, "OK");
    send_response(stream, 200, "OK", "text/html; charset=utf-8", &html).await?;

    Ok(())
}

// async fn handle_script(
//     stream: &mut TcpStream,
//     root: &str,
//     path: &str,
//     headers: &HashMap<String, String>,
//     client_ip: &str,
//     method: &str,
//     body: &str
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let (script_path_str, query_string) = path.split_once('?').unwrap_or((path, ""));
//     let script_path_str = script_path_str.trim_start_matches("/scripts/");
//     let script_path = Path::new(root).join("scripts").join(script_path_str);

//     if !script_path.is_file() {
//         log_request(method, client_ip, path, 404, "Not Found");
//         // return send_error_response(stream, 404, "Not Found").await;
//     }

//     let mut command = Command::new(script_path)
//         .env_clear()
//         .envs(headers)
//         .env("METHOD", method)
//         .env("PATH", path)
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped());

//     query_string.split('&').for_each(|param| {
//         if let Some((key, value)) = param.split_once('=') {
//             command.env(format!("QUERY_{}", key), value);
//         }
//     });

//     if method == "POST" {
//         command.stdin(Stdio::piped());
//     }

//     let mut child = command.spawn()?;
    
//     if method == "POST" {
//         if let Some(mut stdin) = child.stdin.take() {
//             io::copy(&mut body.as_bytes(), &mut stdin).await?;
//         }
//     }

//     let output = child.wait_with_output().await?;
//     let output_status = output.status;
//     let stdout = String::from_utf8_lossy(&output.stdout);
    
//     if output_status.success() {
//         let (headers, response_body) = parse_script_output(&stdout);
        
//         log_request(method, client_ip, script_path_str, 200, "OK");
//         send_script_response(stream, 200, "OK", &headers, &response_body).await?;
//     } else {
//         log_request(method, client_ip, path, 500, "Internal Server Error");
//         // send_error_response(stream, 500, "Internal Server Error").await?;
//     }

//     Ok(())
// }
async fn handle_script(
    stream: &mut TcpStream,
    root: &str,
    path: &str,
    headers: &HashMap<String, String>,
    client_ip: &str,
    method: &str,
    body: &str
) -> Result<(), Box<dyn std::error::Error>> {

    let (script_path_str, query_string) = path.split_once('?').unwrap_or((path, ""));
    let script_path_str = script_path_str.trim_start_matches("/scripts/");
    let script_path = Path::new(root).join("scripts").join(script_path_str);

   
    if !script_path.is_file() {
        log_request(method, client_ip, path, 404, "Not Found");
        // return send_error_response(stream, 404, "Not Found").await;
    }


    let headers = headers.clone(); 
    let method = method.to_string(); 
    let path = path.to_string(); 

    let mut command = Command::new(&script_path)
    .env_clear()
    .envs(&headers)
    .env("METHOD", &method)
    .env("PATH", &path)
    .stdout(tokio::process::Stdio::piped())
    .stderr(tokio::process::Stdio::piped())
    



    query_string.split('&').for_each(|param| {
        if let Some((key, value)) = param.split_once('=') {
            command.env(format!("QUERY_{}", key), value);
        }
    });

    
    if method == "POST" {
        command.stdin(std::process::Stdio::piped());
    }

    // let mut child = command.spawn().await?;
    let mut child = command.spawn().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    if method == "POST" {
        if let Some(mut stdin) = child.stdin.take() {
            io::copy(&mut body.as_bytes(), &mut stdin).await?;
        }
    }

    let output = child.wait_with_output().await?;
    let output_status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    if output_status.success() {
        let (headers, response_body) = parse_script_output(&stdout);
        
        // log_request(method, client_ip, script_path_str, 200, "OK");
        // send_script_response(stream, 200, "OK", &headers, &response_body).await?;
    } else {
        // log_request(method, client_ip, path, 500, "Internal Server Error");
        // send_error_response(stream, 500, "Internal Server Error").await?;
    }

    Ok(())
}

fn parse_script_output(output: &str) -> (HashMap<String, String>, String) {
    let mut headers = HashMap::new();
    let mut body = String::new();
    let mut reading_body = false;
    
    for line in output.lines() {
        if reading_body {
            body.push_str(line);
            body.push('\n');
        } else if line.is_empty() {
            reading_body = true;
        } else if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    (headers, body.trim_end().to_string())
}


async fn send_script_response(
    stream: &mut TcpStream,
    status_code: u32,
    status: &str,
    script_headers: &HashMap<String, String>,
    body: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = format!(
        "HTTP/1.1 {} {}\r\n",
        status_code, status
    );

    let mut content_length_set = false;

    for (key, value) in script_headers {
        if key.to_lowercase() == "content-length" {
            content_length_set = true;
        }
        response.push_str(&format!("{}: {}\r\n", key, value));
    }

    if !content_length_set {
        response.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }

    response.push_str("Connection: close\r\n\r\n");
    response.push_str(body);
    
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

fn get_content_type(path: &Path) -> String {
    match path.extension().and_then(std::ffi::OsStr::to_str) {
        Some("txt") => "text/plain; charset=utf-8".to_string(),
        Some("html") => "text/html; charset=utf-8".to_string(),
        Some("css") => "text/css; charset=utf-8".to_string(),
        Some("js") => "text/javascript; charset=utf-8".to_string(),
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("png") => "image/png".to_string(),
        Some("zip") => "application/zip".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

async fn send_response(stream: &mut TcpStream, status_code: u32, status: &str, content_type: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_code, status, content_type, message.len(), message
    );
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

fn log_request(method: &str, client_ip: &str, path: &str, status_code: u32, status_text: &str) {
    println!("{} {} {} -> {} ({})", method, client_ip, path, status_code, status_text);
}