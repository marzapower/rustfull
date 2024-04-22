use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use rustfull::handlers::{Handler, SimpleHandler};
use rustfull::ThreadPool;

use futures::executor::block_on;
use sea_orm::{DatabaseConnection, EntityTrait};

use entity::prelude::*;

fn main() {
    dotenvy::dotenv().unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(5).unwrap();

    for stream in listener.incoming().take(20) {
        let stream = stream.unwrap();

        pool.execute(move |db: &DatabaseConnection| {
            block_on(handle_connection(&stream, db));
        });
    }

    println!("Gracefully shutting down");
}

async fn handle_connection(mut stream: &TcpStream, db: &DatabaseConnection) {
    // let db = block_on(run()).unwrap();

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let pieces: Vec<_> = request_line.split(' ').collect();

    if pieces.len() < 3 {
        write_response(stream, 500, "ERROR", "<html><body>Error!</body></html>");
        return;
    }

    let http_method = pieces.first().unwrap();
    let uri = pieces.get(1).unwrap();
    let http_version = pieces.get(2).unwrap();

    let mut handlers = Vec::new();
    handlers.push(SimpleHandler::<Users>::new(db));

    Users::find_by_id(1);

    if *http_version == "HTTP/1.1" {
        let mut handled = false;

        for handler in &mut handlers {
            if let Some(result) = handler.handle(http_method, uri).await {
                let json = result.unwrap();
                write_response(stream, 200, "OK", &json);
                handled = true;
                break;
            }
        }

        if !handled {
            println!("[{http_method} - 404] ({uri}): This path is not available");
            write_response(
                stream,
                404,
                "NOT FOUND",
                "<html><body>Not found!</body></html>",
            );
        }
    } else {
        write_response(stream, 500, "ERROR", "<html><body>Error!</body></html>");
    }
}

fn write_response(mut stream: &TcpStream, status: u16, msg: &str, content: &str) {
    let status = format!("HTTP/1.1 {status} {msg}");
    let size = content.len();

    let mut headers = HashMap::new();
    headers.insert("Content-Length", format!("{size}"));
    headers.insert("Content-Type", String::from("application/json"));

    let mut header_str = String::new();
    for (key, val) in &headers {
        header_str = format!("{header_str}\r\n{key}: {val}");
    }

    let response = format!("{status}{header_str}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
