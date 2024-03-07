use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
// The std::io module contains the necessary traits and structs for working with input and output.
// The std::net module contains the necessary traits and structs for working with networking.
// We are using the TcpListener and TcpStream structs to create a server that listens for incoming connections.
// The BufReader struct is used to read the incoming data from the stream.
// The prelude module is a collection of the most common traits and structs that are used in I/O operations.
// We are using the prelude module to import the necessary traits and structs for working with I/O.

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // The bind method is used to bind the server to the address and port.

    for stream in listener.incoming() {
        // The incoming method returns an iterator that yields a new TcpStream for each incoming connection.
        let stream = stream.unwrap();
        // The unwrap method is used to panic if there is an error.
        // In this case, we are using it to panic if we are unable to get the stream from the incoming connection.
        // The stream is a TcpStream that represents the connection to and from the client.

        // We are calling the handle_connection function for each incoming connection.

        handle_connection(stream);
        // The handle_connection function is called with the stream as an argument.
    }
}

// in the main function, we are binding the server to the address and port,
// then we are listening for incoming connections.
// When a connection is received, we are calling the handle_connection function and passing the stream to it.

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}

/*
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "Connection: keep-alive",
    "Pragma: no-cache",
    "Cache-Control: no-cache",
    "sec-ch-ua: \"Chromium\";v=\"122\", \"Not(A:Brand\";v=\"24\", \"Google Chrome\";v=\"122\"",
    "sec-ch-ua-mobile: ?0",
    "sec-ch-ua-platform: \"macOS\"",
    "Upgrade-Insecure-Requests: 1",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,;q=0.8,application/signed-exchange;v=b3;q=0.7",
    "Sec-Fetch-Site: cross-site",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-User: ?1",
    "Sec-Fetch-Dest: document",
    "Accept-Encoding: gzip, deflate, br, zstd",
    "Accept-Language: en-GB,en-US;q=0.9,en;q=0.8",
]
 */
