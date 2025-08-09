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

        println!("Server is running on {}", self.addr)
    }
}