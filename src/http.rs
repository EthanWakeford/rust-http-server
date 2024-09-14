use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub enum Method {
    GET,
}

pub struct RouteConfig {
    pub method: Method,
    pub route: &'static str,
}

fn _print_request(buf_reader: BufReader<&mut TcpStream>) {
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}

pub fn parse_http<'a>(request_line: &String, config: &RouteConfig) -> (&'a str, &'a str) {
    match config.method {
        Method::GET => {
            if &request_line[..] == format!("GET {} HTTP/1.1", config.route) {
                ("HTTP/1.1 200 OK", "index.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "404.html")
            }
        }
    }
}
