use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub enum Method {
    GET,
}

// pub enum RouteHandler {
//     File(&'static str),
//     Controller(fn() -> String),
// }

// TODO: turn into hashmap
pub struct RouteConfig {
    pub method: Method,
    pub route: &'static str,
    pub file: &'static str,
}

fn _print_request(buf_reader: BufReader<&mut TcpStream>) {
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}

pub fn parse_http<'a>(request_line: &String, config: &Vec<RouteConfig>) -> (&'a str, &'a str) {
    let mut request = request_line[..].split_whitespace();
    let method = request.next().unwrap();
    let route = request.next().unwrap();

    for route_config in config {
        match route_config.method {
            Method::GET => {
                if method == "GET" && route == route_config.route {
                    return ("HTTP/1.1 200 OK", route_config.file);
                }
            }
        }
    }
    return ("HTTP/1.1 404 NOT FOUND", "404.html");
}
