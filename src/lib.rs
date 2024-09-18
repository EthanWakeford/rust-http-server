use std::{
    collections::HashMap,
    fs,
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

mod http;
mod threadpool;
use http::{match_controller, parse_request};
pub use http::{Method, RouteConfig, RouteKey};
use threadpool::ThreadPool;

pub fn start_server(
    config: HashMap<RouteKey<'static>, RouteConfig>,
    host: &'static str,
    port: &'static str,
) {
    let address = format!("{host}:{port}");
    let listener = TcpListener::bind(address.clone()).expect("Port Should Bind");

    println!("\n\nServer Now Running on http://{}", address);

    let pool = ThreadPool::new(4);
    let config = Arc::new(config);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let config = Arc::clone(&config);

                pool.execute(move || {
                    handle_connection(stream, config);
                });
            }
            Err(err) => {
                eprintln!("Error connecting to incoming stream: {}", err);
                continue;
            }
        }
    }

    // should never run correct?
    println!("Shutting Down");
}

fn handle_connection(mut stream: TcpStream, config: Arc<HashMap<RouteKey, RouteConfig>>) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    let (request_line, headers, body) = match parse_request(buf_reader) {
        Ok((request_line, headers, body)) => (request_line, headers, body),
        Err(err) => {
            eprintln!("Error parsing request: {}", err);
            (String::new(), vec![], String::new())
        }
    };

    let (status_line, response) = match match_controller(&request_line, &config) {
        Some(route_config) => (
            "HTTP/1.1 200 OK",
            (route_config.controller)(&request_line, headers, body),
        ),
        None => (
            "HTTP/1.1 404 NOT FOUND",
            render_file("404.html")(&request_line, headers, body),
        ),
    };

    send_response(stream, status_line, response);
}

fn send_response(mut stream: TcpStream, status_line: &str, contents: String) {
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // I dont think this can fail?
    stream
        .write_all(response.as_bytes())
        .expect("Writing to stream should not fail");
}

pub fn render_file<'a>(
    filename: &'a str,
) -> Box<impl Fn(&String, Vec<String>, String) -> String + 'a> {
    Box::new(move |_: &String, _: Vec<String>, _: String| {
        fs::read_to_string(filename).unwrap_or_else(|err| {
            eprintln!("Error reading file: {} at \"{}\"", err, filename);
            "HTTP/1.1 500 Internal Server Error".to_string()
        })
    })
}

pub fn make_response<'a>(
    message: &'a str,
) -> Box<impl Fn(&String, Vec<String>, String) -> String + 'a> {
    Box::new(move |_: &String, _: Vec<String>, _: String| message.to_string())
}

#[cfg(test)]
mod tests {}
