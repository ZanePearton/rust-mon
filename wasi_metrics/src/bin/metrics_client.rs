use std::net::TcpStream;
use std::io::Write;
use sysinfo::{System, SystemExt, ProcessorExt};
use std::time::Duration;
use std::thread;

fn send_data(data: String) {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            if let Err(e) = stream.write(data.as_bytes()) {
                eprintln!("Failed to send data: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Could not connect to server: {}", e);
        }
    }
}

fn main() {
    let mut system = System::new_all();

    // Set a periodic collection of metrics (e.g., every 5 seconds)
    loop {
        // Refresh system data to make sure metrics are up to date
        system.refresh_all();

        // Collect the system metrics into a formatted string
        let metrics = format!(
            "Total memory: {} KB\nAvailable memory: {} KB\nCPU load: {}%\n",
            system.total_memory(),
            system.available_memory(),
            system.processors()[0].cpu_usage()
        );

        // Attempt to send the data to the server
        send_data(metrics);

        // Wait for 5 seconds before collecting metrics again
        thread::sleep(Duration::from_secs(5));
    }
}
