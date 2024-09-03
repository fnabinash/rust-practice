// Write a function that attempts to connect to a server and returns a custom error type if the connection fails.

use std::net::TcpStream;
use std::io;
use std::fmt;

#[derive(Debug)]
enum ConnectionError {
    ConnectionFailed(String),
    InvalidAddress(String),
    IoError(io::Error),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::ConnectionFailed(addr) => write!(f, "Failed to connect to server at {}", addr),
            ConnectionError::InvalidAddress(addr) => write!(f, "Invalid server address: {}", addr),
            ConnectionError::IoError(err) => write!(f, "I/O Error: {}", err),
        }
    }
}

impl From<io::Error> for ConnectionError {
    fn from(err: io::Error) -> Self {
        ConnectionError::IoError(err)
    }
}

fn connect_to_server(address: &str) -> Result<TcpStream, ConnectionError> {
    if address.is_empty() {
        return Err(ConnectionError::InvalidAddress(address.to_string()));
    }

    match TcpStream::connect(address) {
        Ok(stream) => Ok(stream),
        Err(_) => Err(ConnectionError::ConnectionFailed(address.to_string())),
    }
}

fn main() {
    let server_address: &str = "127.0.0.1:8080";

    match connect_to_server(server_address) {
        Ok(_) => println!("Successfully connected to the server at {}", server_address),
        Err(e) => println!("Error: {}", e),
    }
}
