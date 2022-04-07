use std::net::{TcpListener, TcpStream};
use std::fs;
use std::io::prelude::*;
use multi_server::ThreadPool;
//use std::io::{Read, Write};
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status, filename) = match buffer.starts_with(get) {
         true => ("HTTP/1.1 200 OK", "table.html"),
           _  => ("HTTP/1.1 404 NOT FOUND", "error.html"),
    };
       let contents = fs::read_to_string(filename).unwrap();
       let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status, contents.len(), contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("\n\t Listening to connections on port 7878...");
    
    let pool = ThreadPool::new(10).unwrap();

    for stream in listener.incoming().take(5) {
         let stream = stream.unwrap();
         pool.execute(||{
             handle_connection(stream);
         });
    }
}