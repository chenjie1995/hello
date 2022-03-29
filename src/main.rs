use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_info = String::from_utf8_lossy(&buffer[..]);
    let path = request_info
        .lines()
        .next()
        .map(|e| get_request_path(e))
        .unwrap_or_else(|| "/");

    let (status_line, filename) = if path == "/" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 200 OK", path)
    };

    let file_path = format!("./{}", filename);
    println!("{}", file_path);
    let contents = fs::read(file_path).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n",
        status_line,
        contents.len()
    );
    stream.write(response.as_bytes()).unwrap();
    stream.write_all(&contents[..]).unwrap();
    stream.flush().unwrap();
}


fn get_request_path(request_first_line: &str) -> &str {
    let request_first_line_split: Vec<&str> = request_first_line.split_whitespace().collect();
    
    let path = request_first_line_split[1].trim_start_matches('/');

    path
}
