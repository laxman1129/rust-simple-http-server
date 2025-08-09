use super::method::Method;
pub struct Request {
    path: String,
    query_string: Option<String>, // `Option` is used to hold value that may be absent
    method: Method,
}