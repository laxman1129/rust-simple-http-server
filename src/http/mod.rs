pub use request::Request; // this is a re-export, so we can use `http::Request` instead of `http::request::Request`
pub use method::Method; // this is a re-export, so we can use `http::Method` instead of `http::method::Method`
pub use request::ParseError;
pub use query_string::{QueryString,Value as QueryStringValue};

pub mod method;
pub mod request;
mod query_string;