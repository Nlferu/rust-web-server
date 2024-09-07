use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // 127.0.0.1:7878 is localhost -> check in browser
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Iterating over the connections...
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        // 1024 bytes long (allows to store basic requests) -> in production server we would need to make arbitrary buffer
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        // Adding 'b' at start turning string into bytes
        let get = b"GET / HTTP/1.1\r\n";

        if buffer.starts_with(get) {
            let contents = fs::read_to_string("index.html").unwrap();

            // 'from_utf8_lossy' -> converts a slice of bytes to a string including invalid characters
            // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

            // Handling 'Response'
            // Response structure:
            // HTTP-Version Status-Code Reason-Phrase CRLS
            // Headers CRLF
            // message-body
            //
            // ex: HTTP/1.1 200 OK\r\n\r\n

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let contents = fs::read_to_string("404.html").unwrap();

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
}
