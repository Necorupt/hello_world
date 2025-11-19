pub mod request;
pub mod response;

use request::Request;
use response::Response;

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn default_error_handler() -> Response {
    let mut response = Response::new();

    response.status = response::StatusCode::ServerError500;

    return response;
}

fn default_request_callback(_: Request) -> Response {
    let mut response = Response::new();

    response.status = response::StatusCode::NotFound404;

    return response;
}

pub type RequestErrorCallback = fn() -> Response;
pub type RequestCallback = fn(Request) -> Response;

pub struct Server {
    request_error_callback: RequestErrorCallback,
    not_found_callback: RequestCallback,
    routes: HashMap<String, RequestCallback>,
}

impl Server {
    pub fn new() -> Self {
        return Server {
            request_error_callback: default_error_handler,
            not_found_callback: default_request_callback,
            routes: HashMap::new(),
        };
    }

    pub fn add_route(&mut self, _path: String, _handler: RequestCallback) {
        self.routes.insert(_path, _handler);
    }

    pub fn start_serving(&self, _addr_to_listen: &String) -> bool {
        let listner = TcpListener::bind(_addr_to_listen).expect("Failed to bind address");

        println!("Start serving for: {0}", _addr_to_listen);

        for stream in listner.incoming() {
            let stream = stream.unwrap();

            Server::handle_connection(self, stream);
        }

        return true;
    }

    fn mount_directory(&self, _directory: String, _as: String) {

    }

    fn handle_connection(&self, stream: TcpStream) {
        let buff = BufReader::new(&stream);
        let response;

        let data: Vec<_> = buff
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        if data.len() == 0 {
            response = (self.request_error_callback)();
            self.write_response(response, stream);

            return;
        }

        let mut req = Request::new();
        req.parse(data);

        let target_callback = match self.routes.get(&req.path) {
            Some(val) => val,
            None => {
                response = (self.not_found_callback)(req);
                self.write_response(response, stream);

                return;
            }
        };

        response = target_callback(req);
        self.write_response(response, stream);
    }

    fn write_response(&self, _response: Response, mut stream: TcpStream) {
        let data: String = _response.to_string();

        stream.write_all(data.as_bytes()).unwrap();
    }
}
