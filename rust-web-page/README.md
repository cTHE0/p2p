# Simple Rust Web Page Project

This project sets up a simple web server using Rust to serve a basic HTML page on localhost.

## Project Structure

```
rust-web-page
├── src
│   ├── main.rs
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

Make sure you have Rust and Cargo installed on your machine. You can install them from [the official Rust website](https://www.rust-lang.org/tools/install).

### Running the Web Server

1. Clone the repository or download the project files.
2. Navigate to the project directory:
   ```
   cd rust-web-page
   ```
3. Build the project:
   ```
   cargo build
   ```
4. Run the web server:
   ```
   cargo run
   ```
5. Open your web browser and go to `http://localhost:3030` to see the basic HTML page served by the application.

## Dependencies

This project uses the following dependencies:

- `warp` or `actix-web`: A web framework for building web applications in Rust.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.