use super::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::OK,Some("<h1>Test Impl</h1>".to_string()))
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(
                    StatusCode::OK,
                    Some("<h1>Welcome to the Home Page</h1>".to_string()),
                ),
                "/about" => Response::new(StatusCode::OK, Some("<h1>About Us</h1>".to_string())),
                "/contact" => {
                    Response::new(StatusCode::OK, Some("<h1>Contact Us</h1>".to_string()))
                }
                _ => Response::new(
                    StatusCode::NotFound,
                    Some("<h1>Page Not Found</h1>".to_string()),
                ),
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
