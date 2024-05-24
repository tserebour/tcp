use std::net::TcpStream;

fn main(){
    let stream = TcpStream::connect("127.0.0.1:8080");

    match stream{
        Ok(stream) => {
            println!("Connected");
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

