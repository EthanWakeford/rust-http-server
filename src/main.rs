use std::collections::HashMap;

use rust_http_server::{start_server, Method, RouteConfig, RouteKey};

fn main() {
    let config: HashMap<RouteKey<'static>, RouteConfig> = HashMap::from([
        (
            RouteKey("/", Method::GET),
            RouteConfig {
                file: ("index.html"),
            },
        ),
        (
            RouteKey("/other", Method::GET),
            RouteConfig {
                file: ("other.html"),
            },
        ),
    ]);
    start_server(config, "127.0.0.1", "7878");
}
