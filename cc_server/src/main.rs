use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;


mod thread_tools;

use thread_tools::ThreadPool;

mod sys_config;
use sys_config::ClientConfig;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(30);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream)
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let rcv = String::from_utf8_lossy(&buffer).trim().to_string();

    let (rcv, _) = rcv.split_at(rcv.find("}").unwrap() + 1);

    let config: ClientConfig = ClientConfig::to_config(String::from(rcv));
    
    println!("{}", config.to_string());

}