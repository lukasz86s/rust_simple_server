
use server::Server;
use http::Method;
use http::Request;
use std::net::TcpListener;


mod server;
mod http;

fn main() {
    
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;

    // let str = String::from("127.0.0.1:8080");
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
    
}