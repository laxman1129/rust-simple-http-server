# Http server

This is a simple HTTP server for serving static files and handling basic routing, built using rust.


## setup

```shell
cargo new server
cd server
cargo run
```

## implementation

```text
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
```

we'll parse the request line, and process it.
- the request contains a 
  - method, 
  - path, and 
  - query string

## Modules

every file in rust is a module, and the file name is the module name.

- in rust we can organize code into modules
- modules can be nested
- by default the content of a module is private
- we can use `pub` to make a module or its content public
- we can use modname::sub_mod_name::item to access a public item in a submodule
- we can use `super` to access items in the parent module
- we can use `self` to access items in the current module
- we can use `use` keyword to bring items from a module into scope