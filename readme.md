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