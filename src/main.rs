use std::net::{TcpListener, TcpStream};
use std::io::prelude::{Read, Write};

const SERVER_HOST: &str = "127.0.0.1";
const SERVER_PORT: u32 = 8888;

fn main() {

    let server_socket = create_server_socket(SERVER_HOST, SERVER_PORT).expect("Failed to bind server socket.");

    for stream in server_socket.incoming() {
        match stream {
            Ok(s) => {
                handle_request(s).expect("Could not handle request.");
            }
            Err(e) => {
                println!("connection failed! {:?}", e);
            }
        }
    }
}

fn create_server_socket(host: &str, port: u32) -> std::io::Result<TcpListener> {
    let socket = TcpListener::bind(format!("{}:{}", host, port))?;
    println!("Now listening to connections on {} port {}...", host, port);
    Ok(socket)
}

fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
    // Get the client request
    let mut buf: [u8; 1024] = [0; 1024];
    let bytes_read = stream.read(&mut buf)?;
    let request = std::str::from_utf8(&buf[0..bytes_read]).unwrap();
    println!("{}", request);

    // Send HTTP response
    let response = b"HTTP/1.0 200 OK\n\nHello World";
    stream.write_all(response)?;
    Ok(())
}
