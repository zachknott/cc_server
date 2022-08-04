use std::net::TcpStream;
use std::io::Write;

mod sys_config;
use sys_config::ClientConfig;

fn main() {
    let system = ClientConfig::new();
    
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
        
        println!("Connected to server...");
        stream.write(system.to_string().as_bytes()).unwrap();
    } else {
        println!("Couldn't connect to server...");
    }
    
    
}
