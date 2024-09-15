use std::{
    collections::HashMap,
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

pub struct RouteConfig {
    pub method: Method,
    // pub route: &'static str,
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

pub fn parse_http<'a>(
    request_line: &String,
    config: &HashMap<&str, RouteConfig>,
) -> (&'a str, &'a str) {
    let mut request = request_line[..].split_whitespace();
    let method = request.next().unwrap();
    let route = request.next().unwrap();

    match match_route(config, method, route) {
        Some(route_config) => ("HTTP/1.1 200 OK", route_config.file),
        None => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    }
}

fn match_route<'a>(
    config: &'a HashMap<&str, RouteConfig>,
    method: &str,
    route: &str,
) -> Option<&'a RouteConfig> {
    match config.get(route) {
        Some(route_config) => {
            let config_method = match route_config.method {
                Method::GET => "GET",
            };

            if config_method == method {
                Some(route_config)
            } else {
                None
            }
        }
        None => None,
    }
}
