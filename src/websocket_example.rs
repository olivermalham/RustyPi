use std::{net::TcpListener, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

fn main() {
    env_logger::init();
    // Listen on the specified IP and port
    // unwrap means it will panic on a None result
    // (any "unwrap" call means there is no explicit error handling - something to fix in real code)
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();

    // Loop through all incoming connections, spawn a new thread for each
    for stream in server.incoming() {
        // move keyword means that ownership of variables etc is transferred to the thread
        spawn(move || {
            // callback is a closure that borrows a request object, and modifies the provided response object 
            // This is used to read and update the headers on the websocket connection request
            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}", header);
                }

                // Let's add an additional header to our response to the client.
                let headers = response.headers_mut();
                headers.append("MyCustomHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());
                
                // Return an Option<T> object
                Ok(response)
            };
            // (end of the closure)

            // accept the websocket connection, use callback to process the headers.
            // unwrap is used again to get the successful return value, or panic
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            // Message handling infinite loop.
            // In this case it just echos back anything that is received.
            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
