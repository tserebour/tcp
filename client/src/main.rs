use std::net::TcpStream;
use std::io::{Read, Write};
// use std::io::Result;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("connect");

    let mut buffer = [0; 1024];

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Read {} bytes", request);
    

    let a = stream.read(&mut buffer);
    println!("{:?}",a);

    
    

    

   
}

