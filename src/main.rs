#![allow(dead_code)]
// use http::request::Request;
// use http::method::Method;
use http::Request;
use http::Method;
use server::Server;
use crate::website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}
