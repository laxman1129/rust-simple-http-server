use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Result as IOResult, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // this is moved from the display trait to here so that we can handle cases where large data is sent to body
    // using stream directly instead of converting to string

    // TcpStream implements Write trait, so we can use it here as dynamic dispatch.
    // This allows us to pass any type that implements the Write trait, such as TcpStream.
    // vtable is created at runtime, so this is dynamic dispatch.
    // so drawback is that at runtime, program has to look up the vtable to find the correct function to call, which is slower than static dispatch.
    // pub fn send(&self, stream: &mut dyn Write) -> IOResult<()> {

    // this is static dispatch, which is more efficient than dynamic dispatch,
    // compiler creates another function for each type that implements the Write trait, so we can use it here as static dispatch.
    // example send_tcp_stream, send_file, send_buffer, etc.
    // drawback is that the compilation time is longer, and the binary size is larger, as the compiler has to generate code for each type that implements the Write trait.
    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//         let body = match &self.body {
//             Some(b) => b,
//             None => "",
//         };
//         write!(
//             f,
//             "HTTP/1.1 {} {}\r\n\r\n{}",
//             self.status_code,
//             self.status_code.reason_phrase(),
//             body
//         )
//     }
// }
