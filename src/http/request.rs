use super::method::Method;
use std::convert::TryFrom; // using this trait to convert byte array to Request
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

// for converting byte array to string
pub struct Request {
    path: String,
    query_string: Option<String>, // `Option` is used to hold value that may be absent
    method: Method,
}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
//         unimplemented!()
//     }
// }

// this is a trait that allows us to convert a type into another type
impl TryFrom<&[u8]> for Request {
    type Error = ParseError; // custom error type for parsing errors

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf) {
        //     Ok(req) =>{
        //         todo!()
        //     },
        //     Err(_) => return Err(ParseError::InvalidEncoding), // if the byte array cannot be converted to a string, return an error
        // }

        // this is preferred way than above, as it uses the `?` operator to handle errors
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(req) =>{
        //         todo!()
        //     },
        //     Err(e) => return Err(e), // if the byte array cannot be converted to a string, return an error
        // }

        // shorthand for the above
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?; //

        let request = str::from_utf8(buf)?; // when we use `?` directly on from_utf8, compiler looks for the `From` trait implementation for `ParseError`, which we have defined

        todo!()
    }
}

// custom error
pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidProtocol,
    InvalidEncoding,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidMethod => "Invalid method",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidEncoding => "Invalid encoding",
        }
    }
}

// required by Error trait
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// required by Error trait
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

// `From` trait implementation for `ParseError`
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
