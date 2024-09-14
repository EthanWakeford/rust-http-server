use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

mod http;
mod threadpool;
use http::parse_http;
use threadpool::ThreadPool;

pub fn start_server(host: &'static str, port: &'static str) {
    let address = format!("{host}:{port}");
    let listener = TcpListener::bind(address.clone()).expect("Port Should Bind");

    println!("\n\nServer Now Running on http://{}", address);

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // should never run correct?
    println!("Shutting Down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = parse_http(&request_line);

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {}
