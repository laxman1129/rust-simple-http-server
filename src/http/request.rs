use super::QueryString;
use super::method::{Method, MethodError};
use std::convert::TryFrom; // using this trait to convert byte array to Request
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

// for converting byte array to string
#[derive(Debug)]
pub struct Request<'buf> {
    // without lifetime, we can use String instead of &str
    // path: String,
    // query_string: Option<String>, // `Option` is used to hold value that may be absent
    path: &'buf str,
    query_string: Option<QueryString<'buf>>, // `Option` is used to hold value that may be absent
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
//         unimplemented!()
//     }
// }

// this is a trait that allows us to convert a type into another type
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError; // custom error type for parsing errors

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n HEADERS...
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
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

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // request variable is shadowed here, so we can use the same name again
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if "HTTP/1.1" != protocol {
            return Err(ParseError::InvalidProtocol);
        }

        // using the `FromStr` trait implementation for `Method`, which we have defined in `method.rs`,
        // also implemented `From` trait for `ParseError` to convert `MethodError` to `ParseError`
        // provide type so that we can use `FromStr` trait to parse the method
        let method: Method = method.parse()?;

        let mut query_string = None;
        // match path.find("?") {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     },
        //     None => {}
        // }

        // when we are only interested in the Some part of the Option, we can use is_some
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(path[i + 1..].to_string()); // convert to String
        //     path = &path[..i];
        // };

        // we can also use `if let` to match the Some part of the Option, this is more idiomatic in Rust
        if let Some(i) = path.find('?') {
            // before using QueryString impl
            // query_string = Some(&path[i + 1..]); //  Some(path[i + 1..].to_string()) String
            query_string = Some(QueryString::from(&path[i + 1..])); //  Some(path[i + 1..].to_string()) String
            path = &path[..i];
        }

        Ok(Self {
            path, // path.to_string() : String
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((
                &request[..i],     // string token till ' '
                &request[i + 1..], // string token after ' ', excluding the space , since we are delimiting by  ' ' i+1 is safe , in case of random chars like Unicode characters, we may have issue
            ));
        }
    }
    None
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

// `From` trait implementation for `MethodError` to convert to `ParseError`
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        self::ParseError::InvalidMethod
    }
}
