use std::{
    collections::HashMap, fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream},
    
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(&stream);
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let pieces: Vec<_> = request_line.split(" ").collect();

    let http_method = pieces.get(0).unwrap();
    let uri = pieces.get(1).unwrap();
    let http_version = pieces.get(2).unwrap();

    if *http_version == "HTTP/1.1" && *http_method == "GET" {
        match *uri {
            "/" => {
                println!("[{http_method} - 200] ({uri}): We matched the path");
                let html = fs::read_to_string("hello.html").unwrap();
                write_response(stream, 200, "OK", html);
            }

            _ => {
                println!("[{http_method} - 404] ({uri}): This path is not available");
                write_response(stream, 404, "NOT FOUND", String::from("<html><body>Not found!</body></html>"));
            }
        }
    } else {
        write_response(stream, 500, "ERROR", String::from("<html><body>Error!</body></html>"));
    }
}

fn write_response(mut stream: &TcpStream, status: u16, msg: &str, content: String) {
    let status = format!("HTTP/1.1 {status} {msg}");
    let size = content.len();

    let mut headers = HashMap::new();
    headers.insert("Content-Length", format!("{size}"));
    headers.insert("Content-Type", String::from("text/html"));

    let mut header_str = String::from("");
    for (key, val) in headers.iter() {
        header_str = format!("{header_str}\r\n{key}: {val}");
    }

    let response = format!("{status}{header_str}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}