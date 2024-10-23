use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // Start a server listening on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                // Read the data sent from the client
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        println!(
                            "Received data: {}",
                            String::from_utf8_lossy(&buffer[..size])
                        );
                        // Optionally, send a response back to the client
                        let response = "Data received\n";
                        stream.write(response.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Failed to read from connection: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }

    Ok(())
}
