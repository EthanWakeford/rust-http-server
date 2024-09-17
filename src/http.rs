use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    str::FromStr,
};

#[derive(Eq, Hash, PartialEq)]
pub enum Method {
    GET,
    POST,
}

impl FromStr for Method {
    type Err = String;
    fn from_str(method: &str) -> Result<Self, Self::Err> {
        match method {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            // Add more methods here
            _ => Err(format!(
                "Error Parsing HTTP Request Message: Method Not Implemented: {}",
                method
            )),
        }
    }
}

// pub enum RouteHandler {
//     File(&'static str),
//     Controller(fn() -> String),
// }

pub struct RouteConfig {
    // pub method: Method,
    // pub route: &'static str,
    pub file: &'static str,
}

#[derive(Eq, Hash, PartialEq)]
pub struct RouteKey(pub &'static str, pub Method);

fn _print_request(buf_reader: BufReader<&mut TcpStream>) {
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}

pub fn parse_http<'a>(
    request_line: &'a String,
    config: &'a HashMap<RouteKey, RouteConfig>,
) -> (&'a str, &'a str) {
    // dbg!(request_line);
    let mut request = request_line[..].split_whitespace();
    let method = request.next().unwrap();
    let route = request.next().unwrap();
    let notfound = ("HTTP/1.1 404 NOT FOUND", "404.html");

    let method = match method.parse::<Method>() {
        Ok(method) => method,
        Err(err) => {
            eprintln!("{}", err);
            return notfound;
        }
    };

    let key = RouteKey(route, method);
    match config.get(&key) {
        Some(route_config) => ("HTTP/1.1 200 OK", route_config.file),
        None => notfound,
    }
    // notfound
}
