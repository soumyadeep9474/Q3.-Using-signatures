## 3. Using signatures
Extend the solution to Q2 where the clients send the signed messages to the aggregator. And the
aggregator validates the signatures and then computes the average of averages. Any signature
scheme is fine. Set up the premise such that all the processes knows the public keys of all other
processes before hand.



-------------------------------------------------------------------------------------------------









## Build the project using Cargo:

```
cargo build --release
```
## Running the Application
To run the application, use the following command:

```
cargo run --release -- --clients <num_clients>
```
Replace <num_clients> with the desired number of client processes to spawn.

## Testing
To run tests for the project, use the following command:

```
cargo test
```
## Dependencies
The project relies on the following Rust dependencies, managed through Cargo:

ring - A safe and fast Rust library for working with cryptography.
clap - A command-line argument parsing library for Rust.
tokio - An asynchronous runtime for Rust.
tokio-tungstenite - Asynchronous WebSocket communication library for Rust.
futures-util - Utilities for working with futures in Rust.
serde - A Rust library for serializing and deserializing data.
serde_json - JSON support for Serde.
base64 - A Rust library for encoding and decoding base64.
## Configuration
Configure the application by modifying the constants in signature.rs. Replace the placeholder private and public keys with your actual keys.
