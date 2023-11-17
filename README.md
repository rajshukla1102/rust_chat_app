# Chat App

## How you can use this

This is a simple chat application written in Rust. It consists of a server and a client component, allowing users to connect, send messages, and interact in real-time.

## Getting Started

1. Create a new Rust project using the following commands:

    ```bash
    cargo new chat_app
    ```

2. Make sure to add below lines to set up the server and client binaries in `cargo.toml`:
    ```toml
    # Add dependencies

    [[bin]]
    name = "server"
    path = "src/server.rs"

    [[bin]]
    name = "client"
    path = "src/client.rs"
    ```

3. Build your application using the following command:

    ```bash
    cargo build
    ```

4. Open two separate terminals for the server and client.

    - For the server terminal, run:

        ```bash
        cargo run --bin server
        ```

    - For the client terminal, run:

        ```bash
        cargo run --bin client
        ```

5. When prompted in the client terminal, enter your name and hit enter. You will see a message in the server terminal indicating that the user has connected.
