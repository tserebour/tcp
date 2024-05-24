use std::net::TcpStream;

if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
    println!("Connected to the server!");
} else {
    println!("Couldn't connect to server...");
}
