use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use std::io::Result;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let read_buffer = stream.read(&mut buffer);

    match read_buffer {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            println!("Read {} bytes", request);

            let message_to_send = format!("HTTP/1.1 200 OK\r\n\r\nHello Client {}!", &stream.peer_addr().unwrap());

            let response = message_to_send.as_bytes();
            match stream.write(&response){
                Ok(_) => {
                    println!("Wrote {} bytes", response.len());
                }
                Err(e) => {
                    eprintln!("Failed to write to client: {}", e);
                }
            };
        }
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
        }
        
    }

    // println!("{}", stream.peer_addr().unwrap());
    
    
}

fn main() -> std::io::Result<()> {
    let listener:Result<TcpListener> = TcpListener::bind("127.0.0.1:8080");


    match listener {
        Ok(listener) => {
            println!("Listening on 127.0.0.1:8080");
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        std::thread::spawn(|| handle_client(stream));
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    }
                    
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to bind: {}", e);
        }
        
    }

    // accept connections and process them serially
    
    Ok(())
}