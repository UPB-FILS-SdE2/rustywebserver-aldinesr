use std::net::TcpListener;

use tokio::stream;


fn main() {
    // println!("Test ");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("connected");
    }
} 