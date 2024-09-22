use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

#[allow(dead_code)]
fn single_thread_server() {
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

        // 'from_utf8_lossy' -> converts a slice of bytes to a string including invalid characters
        // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        // Handling 'Response'
        // Response structure:
        // HTTP-Version Status-Code Reason-Phrase CRLS
        // Headers CRLF
        // message-body
        //
        // ex: HTTP/1.1 200 OK\r\n\r\n

        // Adding 'b' at start turning string into bytes
        let get = b"GET / HTTP/1.1\r\n";

        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();

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
