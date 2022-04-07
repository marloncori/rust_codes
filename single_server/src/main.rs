use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
//these is a barebone server
// that is, I am working direcly with both
// TCP and HTTP without the help of extern crates

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status, filename) = match buffer.starts_with(get) {
         true => ("HTTP/1.1 200 OK", "index.html"),
           _  => ("HTTP/1.1 404 NOT FOUND", "error.html"),
    };
    println!("\n\t
        Incoming Request: {}",
        // it converts a slice of bytes into string
        String::from_utf8_lossy(&buffer[..])
    );
       let contents = fs::read_to_string(filename).unwrap();
       let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status, contents.len(), contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
}

//HTTP-Version Status-Code Reason-Phrase CRLF
//Headers CRLF
//message-body
//
// ex: HTTP/1.1 200 Ok\r\n\r\n
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("\n\t Listening on port 7878");
    
    for stream in listener.incoming() {
        println!("\t Connection has been stablished!");
        println!("\n\t Open http://localhost:7878 on your browser...");    
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}
