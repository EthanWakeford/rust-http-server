use std::collections::HashMap;

use rust_http_server::{make_response, render_file, start_server, Method, RouteConfig, RouteKey};

fn main() {
    let config = HashMap::from([
        (
            RouteKey("/rest", Method::GET),
            RouteConfig {
                controller: make_response("Hello world this is my rest api"),
            },
        ),
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
    ]);
    start_server(config, "127.0.0.1", "7878");
}
