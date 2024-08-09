// use std::net::TcpListener;
use std::env;
// use tokio::stream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    // println!("Test ");

    let args: Vec<String> = env::args().collect();

    if args.len()!=3{
        eprintln!("use:{} <PORT><ROOT_FOLDER>",args[0]);
    }
    
    let port = &args[1];
    let root_folder = args[2].clone();
    println!("Test Port {} ",args[1]);
    println!("Test root folder {} ",args[2]);
    
  
}

