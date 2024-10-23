# WASI Metrics Collection with Rust

This project demonstrates how to use **WASI (WebAssembly System Interface)** with **Rust** to automate system metrics collection, providing a portable and efficient way to monitor application performance across different environments such as cloud servers, local machines, and edge devices. ⚙️🌍

## Overview

Monitoring the performance of applications is crucial, especially when they run across different platforms. Traditionally, creating low-level system monitoring tools requires platform-specific code, which can be complex to maintain. WASI allows developers to create **cross-platform system tools** that run efficiently on any environment without modifications.

This project uses Rust and WASI to:

- Gather key system metrics (e.g., memory usage, CPU load, disk usage).
- Compile the metrics collection code into WebAssembly for cross-platform compatibility.
- Automate metrics collection and send data to a central server for aggregation. ☁️

## Features

- **Cross-Platform Compatibility**: Write code once and run it on Linux, Windows, or macOS without modifications.
- **Lightweight & Efficient**: Perfect for resource-constrained environments like edge devices or cloud containers.
- **Automated Metrics Collection**: Automatically gather metrics and send them to a server for monitoring. 📊

## Getting Started 🚀

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Wasmtime](https://wasmtime.dev/) or any WASI-compatible WebAssembly runtime

### Setup Instructions

1. **Install Rust and Add WASI Target**

   ```bash
   rustup target add wasm32-wasi
   cargo new wasi_metrics
   cd wasi_metrics
   ```

2. **Add Dependencies**

   Add the `sysinfo` crate to `Cargo.toml` to gather system metrics:

   ```toml
   [dependencies]
   sysinfo = "0.21.2"
   ```

3. **Write Metrics Collection Code**

   Create a Rust file (`src/main.rs`) with the following code to gather system metrics like memory usage, CPU load, and disk information:

   ```rust
   use sysinfo::{System, SystemExt, DiskExt, ProcessorExt};

   fn main() {
       let mut system = System::new_all();
       system.refresh_all();

       println!("Total memory: {} KB", system.total_memory());
       println!("Available memory: {} KB", system.available_memory());

       let load_avg = system.load_average();
       println!(
           "Load Average: 1 min: {}, 5 min: {}, 15 min: {}",
           load_avg.one, load_avg.five, load_avg.fifteen
       );

       for (i, cpu) in system.processors().iter().enumerate() {
           println!("CPU {} load: {}%", i, cpu.cpu_usage());
       }

       for disk in system.disks() {
           println!(
               "Disk {}: Total size: {} KB, Available: {} KB",
               disk.name().to_str().unwrap(),
               disk.total_space() / 1024,
               disk.available_space() / 1024
           );
       }
   }
   ```

4. **Compile to WebAssembly**

   Compile the Rust code to a `.wasm` file:

   ```bash
   cargo build --target wasm32-wasi --release
   ```

5. **Run the WebAssembly Module**

   Use Wasmtime to execute the `.wasm` file:

   ```bash
   wasmtime target/wasm32-wasi/release/wasi_metrics.wasm
   ```

   You should see system metrics printed to the terminal. 🔍

## Automate Metrics Collection to a Server 📈

In a real-world scenario, automating the metrics collection and sending it to a server can be beneficial. Here is an example of how to extend the code to send metrics to a central server:

### Updated Code with Networking

```rust
use std::net::TcpStream;
use std::io::Write;
use sysinfo::{System, SystemExt, ProcessorExt};
use std::time::Duration;
use std::thread;

fn send_data(data: String) {
    let mut retry_count = 0;
    let max_retries = 5;

    while retry_count < max_retries {
        match TcpStream::connect("127.0.0.1:8080") {
            Ok(mut stream) => {
                if let Err(e) = stream.write(data.as_bytes()) {
                    eprintln!("Failed to send data: {}", e);
                } else {
                    println!("Data sent successfully");
                }
                break;
            }
            Err(e) => {
                eprintln!("Could not connect to server: {}. Retrying... ({}/{})", e, retry_count + 1, max_retries);
                retry_count += 1;
                thread::sleep(Duration::from_secs(2));
            }
        }
    }

    if retry_count == max_retries {
        eprintln!("Failed to connect to server after {} attempts", max_retries);
    }
}

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_all();

        let metrics = format!(
            "Total memory: {} KB\nAvailable memory: {} KB\nCPU load: {}%\n",
            system.total_memory(),
            system.available_memory(),
            system.processors()[0].cpu_usage()
        );

        send_data(metrics);
        thread::sleep(Duration::from_secs(5));
    }
}
```

This version adds retry logic for network connection issues, making it resilient in unstable environments. 📡🔄

## Running the Server to Receive Metrics

To receive the metrics from the client, you can set up a simple server to listen on port `8080`:

```rust
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        println!(
                            "Received data: {}",
                            String::from_utf8_lossy(&buffer[..size])
                        );
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
```

## Real-World Use Cases 🔧

- **Cloud and Edge Monitoring**: Collect performance metrics for applications running across cloud and edge environments.
- **Containerized Environments**: Use in Docker or Kubernetes for efficient monitoring with minimal resource usage.
- **IoT Device Monitoring**: Monitor hardware metrics across various IoT devices with a single codebase. 🌐📟

## Why WASI and Rust? 🚀

- **Cross-Platform Consistency**: One codebase for all environments without the hassle of platform-specific modifications.
- **Security**: WASI ensures your monitoring tool runs securely, even in untrusted environments.
- **Efficiency**: Rust's performance combined with WASI's portability makes this an ideal solution for low-level system automation. ⚡

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributions 🤝

Contributions are welcome! Feel free to open issues or submit pull requests to enhance functionality or add new features.

## Acknowledgements

- Thanks to the Rust and WebAssembly communities for providing great tools to make cross-platform system monitoring easier. 🙌

