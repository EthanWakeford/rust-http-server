use rust_http_server::{start_server, Method, RouteConfig};

fn main() {
    let root = RouteConfig {
        method: Method::GET,
        route: "/",
        file: ("index.html"),
    };
    let other = RouteConfig {
        method: Method::GET,
        route: "/other",
        file: ("other.html"),
    };

    let config = vec![root, other];

    start_server(config, "127.0.0.1", "7878");
}
