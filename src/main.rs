use rust_http_server::{start_server, Method, RouteConfig};

fn main() {
    let var_name = RouteConfig {
        method: Method::GET,
        route: "/",
        file: ("index.html"),
    };
    let config = var_name;
    start_server("127.0.0.1", "7878", config);
}
