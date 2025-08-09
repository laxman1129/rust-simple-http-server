pub use request::Request; // this is a re-export, so we can use `http::Request` instead of `http::request::Request`
pub use method::Method; // this is a re-export, so we can use `http::Method` instead of `http::method::Method`

pub mod method;
pub mod request;