use std::io::Read; // using this trait to read from the stream
use std::net::TcpListener;

use crate::http::Request; // using `crate` to refer to the root of the current crate
use std::convert::TryFrom; // using this trait to convert byte array to Request
use std::convert::TryInto; // using this trait to convert byte array to Request
/**
 * Server struct definition
 */
pub struct Server {
    addr: String,
}

/**
 * Implementation of the Server struct
 */
impl Server {
    /**
     * new is a function as it does not have self as the first parameter, so it is a static method.
     */
    pub fn new(addr: String) -> Self {
        // `Server` => we can use `Server`, in place of `Self` ; but `Self` is a convention to refer to the type itself
        Self { addr } // swapping `Server` with `Self` is also fine
    }

    /**
     * run is an instance method as it has self as the first parameter.
     */
    pub fn run(self) {
        // we are using self here, so we can access the instance variables
        // not using &self as we want to take ownership of the instance
        // not using &mut self as we are not modifying the instance

        println!("Server is running on {}", self.addr);
        // returns results => error handling
        // unwrap() is used to panic if there is an error, and terminates the program
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // let res = listener.accept(); // we cannot use `unwrap()` here as it will terminate the program if there is an error

            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    // adding underscore to ignore the variables
                    println!("=====>New connection established from: {}", _addr);

                    // using read trait from std::io which has implementation for TcpStream
                    let mut buffer = [0; 1024]; // mutable buffer to read the stream
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request: {}", String::from_utf8_lossy(&buffer));


                            // Request::try_from(&[buffer]); // this results in an error as we are passing a slice of bytes, not a byte array
                            // Request::try_from(&buffer as &[u8]); // one way

                            // this trait is automatically implemented by the compiler, as specified in the docs.
                            // we have to add use std::convert::TryInto; at the top of the file to use this trait
                            // let res: &Result<Request, _> = &buffer[..].try_into(); // have to specify the type we want to convert to, as this is a generic function

                           match Request::try_from(&buffer[..]){
                               Ok(request)=>{
                                    dbg!(request);
                               },
                               Err(e) => println!("Failed to convert buffer to Request: {:?}", e),
                           } // this is the preferred way to pass a slice of bytes, [..] => all elements of the array
                        }
                        Err(e) => println!("Failed to read from stream: {}", e),
                    }
                }
                Err(e) => println!("Failed to accept connection: {}", e),
            }
        }
    }
}
