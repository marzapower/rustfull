use std::collections::HashMap;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use rustfull::handlers::{Handler, SimpleHandler};

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, EntityTrait};

use entity::prelude::*;

#[tokio::main(worker_threads = 5)]
async fn main() {
    dotenvy::dotenv().unwrap();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let db: DatabaseConnection = Database::connect(&database_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(stream, db.clone()));
    }
}

async fn handle_connection(mut stream: TcpStream, db: DatabaseConnection) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next_line().await.unwrap().unwrap();

    let pieces: Vec<_> = request_line.split(' ').collect();

    if pieces.len() < 3 {
        write_response(
            &mut stream,
            500,
            "ERROR",
            "<html><body>Error!</body></html>",
        ).await;
        return;
    }

    let http_method = pieces.first().unwrap();
    let uri = pieces.get(1).unwrap();
    let http_version = pieces.get(2).unwrap();

    let mut handlers = Vec::new();
    handlers.push(SimpleHandler::<Users>::new(&db));

    Users::find_by_id(1);

    if *http_version == "HTTP/1.1" {
        let mut handled = false;

        for handler in &mut handlers {
            if let Some(result) = handler.handle(http_method, uri).await {
                let json = result.unwrap();
                write_response(&mut stream, 200, "OK", &json).await;
                handled = true;
                break;
            }
        }

        if !handled {
            println!("[{http_method} - 404] ({uri}): This path is not available");
            write_response(
                &mut stream,
                404,
                "NOT FOUND",
                "<html><body>Not found!</body></html>",
            )
            .await;
        }
    } else {
        write_response(
            &mut stream,
            500,
            "ERROR",
            "<html><body>Error!</body></html>",
        )
        .await;
    }
}

async fn write_response(stream: &mut TcpStream, status: u16, msg: &str, content: &str) {
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

    stream.write_all(response.as_bytes()).await.unwrap();
}
