use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Error, Read},
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
    pub controller: Box<dyn Fn(&String, Vec<String>, String) -> String + Sync + Send>,
}

#[derive(Eq, Hash, PartialEq)]
pub struct RouteKey<'a>(pub &'a str, pub Method);

pub fn parse_request(
    mut buf_reader: BufReader<&mut TcpStream>,
) -> Result<(String, Vec<String>, String), Error> {
    let mut request_line = String::new();
    let mut headers = Vec::new();
    let mut body = String::new();

    buf_reader.read_line(&mut request_line)?;

    // Read headers
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line)?;
        let line = line.trim_end().to_string();

        if line.is_empty() {
            // End of headers
            break;
        } else {
            headers.push(line);
        }
    }

    // Determine Content-Length
    let content_length = headers
        .iter()
        .find_map(|header| {
            if header.to_lowercase().starts_with("content-length:") {
                header["content-length:".len()..]
                    .trim()
                    .parse::<usize>()
                    .ok()
            } else {
                None
            }
        })
        .unwrap_or(0);

    // Read body if Content-Length is specified
    if content_length > 0 {
        let mut buffer = vec![0; content_length];
        buf_reader.read_exact(&mut buffer)?;
        body = String::from_utf8_lossy(&buffer).to_string();
    }

    Ok((request_line, headers, body))
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
