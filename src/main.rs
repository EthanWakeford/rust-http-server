use std::collections::HashMap;

use rust_http_server::{start_server, Method, RouteConfig};

fn main() {
    let config = HashMap::from([
        (
            "/",
            RouteConfig {
                method: Method::GET,
                file: ("index.html"),
            },
        ),
        (
            "/other",
            RouteConfig {
                method: Method::GET,
                file: ("other.html"),
            },
        ),
    ]);

    start_server(config, "127.0.0.1", "7878");
}
