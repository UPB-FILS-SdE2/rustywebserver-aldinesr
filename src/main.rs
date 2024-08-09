// use std::net::TcpListener;

// use tokio::stream;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

fn main() -> std::io::Result<()> {
   

    // println!("Test ");
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // for stream in listener.incoming(){
    //     let stream = stream.unwrap();
    //     println!("connected");
    // }

let args:Vec<String> = env::args().collect();
if args.len()!=3{
    eprintln!("Usage:{} <PORT> <ROOT_FOLDER>",args[0]);
    std::process::exit(1);

}


let port = &args[1];
let root_folder = &args[2];

let listner = TcpListener::bind(format!("0.0.0.0:{}",port))?;


for stream in listner.incoming(){
    let stream=stream?;
    let root_folder = root_folder.to_string();
    thread::spawn(move||
    {
       
    });

}

    Ok(())
  
} 

fn connections(mut stream: TcpStream, root_folder: String) -> std::io::Result<()> {

    let mut buff = [0;1024];

    stream.read(&mut buff)?;

    let request = String ::from_utf8_lossy(&buff);
    let (method,path)=requests(&request);

    let full_path = Path::new(&root_folder).join(&path.trim_start_matches('/'));
      if path.starts_with("/..") || !full_path.starts_with(root_folder) {
     
        let response = b"HTTP/1.1 403 Forbidden\r\nConnection: close\r\n\r\n<html>403 Forbidden</html>";
        stream.write_all(response)?;
    } else {
        match fs::read(&full_path) {
            Ok(content) => {
                let content_type = get_extensions(&full_path);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    content_type, content.len()
                );
                stream.write_all(response.as_bytes())?;
                stream.write_all(&content)?;
            }
            Err(_) => {
      
                let response = b"HTTP/1.1 404 Not Found\r\nConnection: close\r\n\r\n<html>404 Not Found</html>";
                stream.write_all(response)?;
            }
        }
    }
    Ok(())
}

fn requests(request:&str)->(&str,&str){
    let mut lines = request.lines();
    let request_line =lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");
    (method, path)
}

fn get_extensions(path: &Path)->&'static str{
    match path.extension().and_then(|ext| ext.to_str()) {
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