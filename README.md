# Rust HTTP Server

Simple Multithreaded HTTP server written in rust built reading the book "The Rust Programming Language"

todo

- build func that generates config from static directory
- accept json content

## Example Usage

```
use rust_http_server::{make_response, render_file, start_server, Method, RouteConfig, RouteKey};
use std::collections::HashMap;

fn main() {
    let config = HashMap::from([
        (
            RouteKey("/", Method::GET),
            RouteConfig {
                controller: render_file("index.html"),
            },
        ),
        (
            RouteKey("/other", Method::GET),
            RouteConfig {
                controller: render_file("other.html"),
            },
        ),
        (
            RouteKey("/other_dir", Method::GET),
            RouteConfig {
                controller: render_file("./static/index.html"),
            },
        ),
        (
            RouteKey("/rest", Method::GET),
            RouteConfig {
                controller: make_response("Hello world this is my rest api"),
            },
        ),
        (
            RouteKey("/my_controller", Method::GET),
            RouteConfig {
                controller: Box::new(x),
            },
        ),
        (
            RouteKey("/read_request", Method::POST),
            RouteConfig {
                controller: Box::new(read_request),
            },
        ),
    ]);

    start_server(config, "127.0.0.1", "7878");
}

fn x(_: &String, _: Vec<String>, _: String) -> String {
    "blah blah".to_string()
}

fn read_request(request: &String, headers: Vec<String>, body: String) -> String {
    let response = format!(
        "Request: {request}\n Headers:{:?}\n Body:{:?}",
        headers, body
    );
    response
}
```
