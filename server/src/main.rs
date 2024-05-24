use std::net::{TcpListener, TcpStream};

use std::io::Result;

fn handle_client(stream: TcpStream) {

    println!("{}", stream.peer_addr().unwrap());
    
}

fn main() -> std::io::Result<()> {
    let listener:Result<TcpListener> = TcpListener::bind("127.0.0.1:8080");


    match listener {
        Ok(listener) => {
            println!("Listening on 127.0.0.1:8080");
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        handle_client(stream);
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