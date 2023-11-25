use std::io::prelude::*;
use std::fs::File;
use std::net::{TcpListener, TcpStream};

mod thread;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Replace invalid chracters as \U+FFFD
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // if block can have value
    let filepath = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        "static/hello.html"
    } else {
        "static/404.html"
    };

    let mut file = File::open(filepath).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = thread::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
