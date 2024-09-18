use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    str::FromStr,
};

use crate::render_file;

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
            _ => Err(format!(
                "Error Parsing HTTP Request Message: Method Not Implemented: {}",
                method
            )),
        }
    }
}

pub struct RouteConfig {
    pub controller: Box<dyn Fn() -> String + Sync + Send>,
}

#[derive(Eq, Hash, PartialEq)]
pub struct RouteKey<'a>(pub &'a str, pub Method);

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
) -> Option<&'a RouteConfig> {
    // dbg!(request_line);
    let mut request = request_line[..].split_whitespace();
    let method = request.next().unwrap();
    let route = request.next().unwrap();

    let method = match method.parse::<Method>() {
        Ok(method) => method,
        Err(err) => {
            eprintln!("{}", err);
            return None;
        }
    };

    match config.get(&RouteKey(route, method)) {
        Some(route_config) => Some(route_config),
        None => None,
    }
}
