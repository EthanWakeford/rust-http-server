use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

mod http;
mod threadpool;
use http::parse_http;
pub use http::{Method, RouteConfig};
use threadpool::ThreadPool;

pub fn start_server(config: Vec<RouteConfig>, host: &'static str, port: &'static str) {
    let address = format!("{host}:{port}");
    let listener = TcpListener::bind(address.clone()).expect("Port Should Bind");

    println!("\n\nServer Now Running on http://{}", address);

    let pool = ThreadPool::new(4);

    let config = Arc::new(config);

    for stream in listener.incoming() {
        let config = Arc::clone(&config);

        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream, config);
        });
    }

    // should never run correct?
    println!("Shutting Down");
}

fn handle_connection(mut stream: TcpStream, config: Arc<Vec<RouteConfig>>) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = parse_http(&request_line, &config);

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error reading file: {} at \"{}\"", err, filename);
            "HTTP/1.1 500 Internal Server Error".to_string()
        }
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {}
