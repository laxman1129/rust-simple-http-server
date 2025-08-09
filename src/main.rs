fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

/**
* Server struct definition
*/
struct Server {
    addr: String,
}

/**
* Implementation of the Server struct
*/
impl Server {
    /**
     * new is a function as it does not have self as the first parameter, so it is a static method.
     */
    fn new(addr: String) -> Self { // `Server` => we can use `Server`, in place of `Self` ; but `Self` is a convention to refer to the type itself
        Self { addr } // swapping `Server` with `Self` is also fine
    }

    /**
     * run is an instance method as it has self as the first parameter.
     */
    fn run(self) {
        // we are using self here, so we can access the instance variables
        // not using &self as we want to take ownership of the instance
        // not using &mut self as we are not modifying the instance

        println!("Server is running on {}", self.addr)
    }
}

struct Request{
    path : String,
    query_string: Option<String>, // `Option` is used to hold value that may be absent
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}






































