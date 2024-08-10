// use std::net::TcpListener;
use std::env;
use std::fs;
// use tokio::stream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    // println!("Test ");

    let args: Vec<String> = env::args().collect();

    if args.len()!=3{
        eprintln!("Usage:{} <PORT><ROOT_FOLDER>",args[0]);
        std::process::exit(1);
    }
    
    let port = &args[1];
    let root_folder = args[2].clone();
    // println!("Test Port {} ",args[1]);
    // println!("Test root folder {} ",args[2]);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.expect("error with bind port");
    loop{
        let(mut socket,_)=listener.accept().await.expect("connection declined");
        let root_folder = root_folder.clone();

     tokio::spawn(async move{
        let mut buff = [0;1024];
        let n = socket.read(&mut buff).await.expect("cant read from socket");
        if n>0{
            socket.write_all(&buff[0..n]).await.expect("cant write into socket");
        }
     });
    }
}

