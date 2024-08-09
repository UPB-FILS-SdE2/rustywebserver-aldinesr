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