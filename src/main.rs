mod http_server;

use http_server::Server;
use std::fs;

use crate::http_server::{
    request::Request,
    response::{Response, StatusCode},
};

fn main_page(_request: Request) -> Response {
    let mut response = Response::new();

    let data =
        fs::read_to_string("public/index.html").expect("Should have been able to read the file");

    response.content = data;
    response.status = StatusCode::Ok200;
    response.add_header(String::from("Server"), String::from("tiny-http(rust)"));
    response.add_header(String::from("Content-type"), String::from("text/html; charset=UTF-8"));

    return response;
}

fn menu_page(_request: Request) -> Response {
    let mut response = Response::new();

    let data =
        fs::read_to_string("public/menu.html").expect("Should have been able to read the file");

    response.content = data;
    response.status = StatusCode::Ok200;

    return response;
}

fn main() {
    let mut srv: Server = Server::new();

    srv.add_route(String::from("/"), main_page);
    srv.add_route(String::from("/menu"), menu_page);

    srv.start_serving(&String::from("192.168.1.200:10800"));
}
