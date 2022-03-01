use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        std::thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; //1024个0
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));
    let (res_page, res_code) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("hello.html", 200)
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(10));
        ("hello.html", 200)
    } else {
        ("404.html", 404)
    };
    let contents = fs::read_to_string(res_page).unwrap();
    let response = format!(
        "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
        res_code,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
