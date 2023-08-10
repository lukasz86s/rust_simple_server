#![allow(dead_code)]

use server::Server;
use http::Method;
use http::Request;
// use std::net::TcpListener;
use std::env;
use std::path::Path;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("public");
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path.to_string_lossy().into_owned() );

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
    
}