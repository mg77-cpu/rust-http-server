use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn main() {
    // Binding the TcpListener to the specified address
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Listening for incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1084]; // Initialize buffer with zeros
    stream.read(&mut buffer).unwrap();

    let get = b"Get / HTTP1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("app.html").expect("Unable to read file.");

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404-page.html").expect("Unable to read file.");

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
