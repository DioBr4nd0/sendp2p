# SendP2P

A peer-to-peer file transfer tool built with Rust and Iroh that enables secure and direct file sharing between computers.

## How It Works

SendP2P uses the Iroh protocol for peer-to-peer file transfer. When sending a file:
1. The sender generates a unique ticket for the file
2. This ticket contains the file hash and node address
3. The receiver uses this ticket to download the file directly from the sender, hence no storage on servers making it free.

## Usage

### To Send a File:
```bash
cargo run send <path_to_file>
```

### To Receive a File:
```bash
cargo run receive <ticket_id> <output_filename>
```

## Examples

### Sending a File:
```bash
cargo run send Cargo.toml
```
This will generate a ticket ID that you can share with the receiver.

### Receiving a File:
```bash
cargo run receive blobaafshpq4bqn2vde42t3srm6tn6bikjbt7hppl6bfidh56v5id255gaaaacwghvhebd6mccn5eqoa5q4k5jru5ea25judk52e744m5gc6jsk4y Cargo.toml
```

## Features

- Direct peer-to-peer file transfer
- No intermediate server required
- Secure file transfer using Iroh protocol
- Simple command-line interface
- Works across different networks

## Reference:

- https://www.iroh.computer/docs/quickstart