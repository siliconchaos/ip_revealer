# IP Revealer

IP Revealer is a simple Rust program that displays both the external (WAN) IP address and the local IP address of the machine it's running on. It now supports command-line options for more flexible usage.

## Features

- Fetches and displays the external (WAN) IP address
- Retrieves and shows the local IP address
- Supports command-line options for selective display of IP addresses
- Offers a raw output mode for easy integration with other tools
- Simple and easy to use

## Installation

To use IP Revealer, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/ip_revealer.git
   cd ip_revealer
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

After building the project, you can run IP Revealer using:

```
cargo run --release -- [OPTIONS]
```

Or, you can run the compiled binary directly:

```
./target/release/ip_revealer [OPTIONS]
```

### Command-line Options

- `--wan`: Display only the WAN (external) IP address
- `--local`: Display only the local IP address
- `--raw`: Display the IP address(es) without labels

Note: `--wan` and `--local` are mutually exclusive. If neither is specified, both IP addresses will be displayed.

### Examples

1. Display both IP addresses (default behavior):
   ```
   ip_revealer
   ```
   Output:
   ```
   External (WAN) IP: 203.0.113.1
   Local IP: 192.168.1.2
   ```

2. Display only the WAN IP:
   ```
   ip_revealer --wan
   ```
   Output:
   ```
   External (WAN) IP: 203.0.113.1
   ```

3. Display only the local IP:
   ```
   ip_revealer --local
   ```
   Output:
   ```
   Local IP: 192.168.1.2
   ```

4. Display the WAN IP in raw format (useful for scripting):
   ```
   ip_revealer --wan --raw
   ```
   Output:
   ```
   203.0.113.1
   ```

5. Pipe the raw WAN IP to another command (e.g., copying to clipboard on macOS):
   ```
   ip_revealer --wan --raw | pbcopy
   ```

## Dependencies

IP Revealer uses the following crates:

- `clap`: For parsing command-line arguments
- `reqwest`: For making HTTP requests to fetch the external IP
- `local_ip_address`: For retrieving the local IP address

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.