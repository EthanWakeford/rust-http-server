use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Lines},
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
            _ => Err(format!(
                "Error Parsing HTTP Request Message: Method Not Implemented: {}",
                method
            )),
        }
    }
}

pub struct RouteConfig {
    pub controller: Box<dyn Fn(&String, Vec<String>, Vec<String>) -> String + Sync + Send>,
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

pub fn parse_request(
    mut request: Lines<BufReader<&mut TcpStream>>,
) -> (String, Vec<String>, Vec<String>) {
    let request_line = request
        .next()
        .expect("Request should not be empty")
        .expect("Should be able to parse message");

    let mut headers = Vec::new();
    let mut body = Vec::new();
    let mut header_finished = false;

    // loop {
    //     match request.next() {
    //         None => break,
    //         Some(line) => {
    //             println!("looping");
    //             if !header_finished {
    //                 println!("in header");
    //                 // Can't for the life of me figure out why this would be error
    //                 let header = line.expect("Should be able to unwrap Headers");

    //                 // headers end when empty entry
    //                 if header.is_empty() {
    //                     println!("header is empty");
    //                     // break;
    //                     header_finished = true;
    //                     // continue;
    //                 } else {
    //                     println!("header pushed");
    //                     headers.push(header);
    //                 }
    //             } else {
    //                 println!("in body");
    //                 let body_line = line.expect("Should be able to unwrap body");

    //                 body.push(body_line);
    //             }
    //             println!("end of loop");
    //         }
    //     }
    // }

    while let Some(line) = request.next() {
        println!("looping");
        if !header_finished {
            println!("in header");
            // Can't for the life of me figure out why this would be error
            let header = line.expect("Should be able to unwrap Headers");

            // headers end when empty entry
            if header.is_empty() {
                println!("header is empty");
                // break;
                header_finished = true;
                // continue;
            } else {
                println!("header pushed");
                headers.push(header);
            }
        } else {
            println!("in body");
            let body_line = line.expect("Should be able to unwrap body");

            body.push(body_line);
        }
        println!("end of loop");
    }

    // println!("creating body");
    // while let Some(body_line) = request.next() {
    // println!("in body");
    // let body_line = body_line.expect("Should be able to unwrap body");
    //
    // body.push(body_line);
    // }

    println!("returning");
    (request_line, headers, body)
}

pub fn match_controller<'a>(
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
