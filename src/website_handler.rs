use super::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};
use std::fs;
use std::path::StripPrefixError;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        // echo "GET style.css HTTP/1.1\r\n" | nc 127.0.0.1 8080
        // echo "GET /../cargo.toml HTTP/1.1\r\n" | nc 127.0.0.1 8080

        // this returns resolved filepath and we'll check if the path does not start with public_path
        // then we return None
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack detected! Path: {}", path.display());
                    None
                }
            }
            Err(_) => None,
        }

        // fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::OK,Some("<h1>Test Impl</h1>".to_string()))
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(
                    StatusCode::OK,
                    // Some("<h1>Welcome to the Home Page</h1>".to_string()),
                    self.read_file("index.html"),
                ),
                "/about" => Response::new(
                    StatusCode::OK,
                    // Some("<h1>About Us</h1>".to_string())
                    self.read_file("about.html"),
                ),
                "/contact" => {
                    Response::new(StatusCode::OK, Some("<h1>Contact Us</h1>".to_string()))
                }
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::OK, Some(content)),
                    None => Response::new(
                        StatusCode::NotFound,
                        Some("<h1>Page Not Found</h1>".to_string()),
                    ),
                },
            },
            Method::POST => match request.path() {
                "/submit" => {
                    Response::new(StatusCode::OK, Some("<h1>Form Submitted</h1>".to_string()))
                }
                _ => Response::new(
                    StatusCode::NotFound,
                    Some("<h1>Page Not Found</h1>".to_string()),
                ),
            },
            Method::PUT => match request.path() {
                "/update" => Response::new(
                    StatusCode::OK,
                    Some("<h1>Update Successful</h1>".to_string()),
                ),
                _ => Response::new(
                    StatusCode::NotFound,
                    Some("<h1>Page Not Found</h1>".to_string()),
                ),
            },
            Method::DELETE => match request.path() {
                "/delete" => Response::new(
                    StatusCode::OK,
                    Some("<h1>Delete Successful</h1>".to_string()),
                ),
                _ => Response::new(
                    StatusCode::NotFound,
                    Some("<h1>Page Not Found</h1>".to_string()),
                ),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
