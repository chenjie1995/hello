use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(16);

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
    let request_path = request_info
        .lines()
        .next()
        .map(|e| get_request_path(e))
        .unwrap_or("/");

    let file_path = format!(".{}", request_path);

    let (status_line, file_path) = if request_path == "/" {
        ("HTTP/1.1 200 OK", "./index.html")
    } else if fs::metadata(&file_path).is_ok() {
        ("HTTP/1.1 200 OK", file_path.as_str())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./404.html")
    };

    let contents = fs::read(file_path).unwrap_or(vec![]);

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
    
    request_first_line_split[1]
}
