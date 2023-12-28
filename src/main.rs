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
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    let request_line: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let user_agent_header = "User-Agent:";
    let user_agent = request_line
        .iter()
        .find(|line| line.starts_with(user_agent_header))
        .unwrap()
        .strip_prefix(user_agent_header)
        .unwrap()
        .trim();
    // println!("User-Agent: {:#?}", user_agent);

    // println!("Request: {:#?}", request_line);
    // println!("{:#?}", request_line[0]);

    let split_line: Vec<_> = request_line[0].split(" ").collect();
    let method = split_line[0];
    let mut path = split_line[1].to_string();
    // let protocol = split_line[2];
    if method == "GET" && path == "/" {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
    if path.starts_with("/echo/") {
        path = path.replace("/echo", "");
        if path.starts_with("/") {
            path.remove(0);
        }
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            path.len(),
            path
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
    else if path.starts_with("/user-agent"){
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent.len(),
                user_agent
            );
        stream.write_all(response.as_bytes()).unwrap();
    }
    else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }

    // println!("Request: {:#?}", request_line);
    // println!("Split: {:#?}", split_line);
    // println!("Method: {:#?}", method);
    // println!("Path: {:#?}", path);
    // println!("Protocol: {:#?}", protocol);
}
