// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {:#?}", http_request);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("Request: {:#?}", request_line);
    if request_line == "GET / HTTP/1.1" {
        stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    } else {
        stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n").unwrap();
    }
}
