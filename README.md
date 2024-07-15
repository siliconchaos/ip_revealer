# IP Revealer

IP Revealer is a simple Rust program that displays both the external (WAN) IP address and the local IP address of the machine it's running on.

## Features

- Fetches and displays the external (WAN) IP address
- Retrieves and shows the local IP address
- Simple and easy to use

## Installation

To use IP Revealer, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/siliconchaos/ip_revealer.git
   cd ip_revealer
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

After building the project, you can run IP Revealer using:

```
cargo run --release
```

Or, you can run the compiled binary directly:

```
./target/release/ip_revealer
```

The program will display output similar to this:

```
External (WAN) IP: 203.0.113.1
Local IP: 192.168.1.2
```

## Dependencies

IP Revealer uses the following crates:

- `reqwest`: For making HTTP requests to fetch the external IP
- `local_ip_address`: For retrieving the local IP address

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.