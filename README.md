# RustyChat

RustyChat is a simple, real-time chat application built with **Rust** using the **socketioxide** library for WebSocket communication. It allows users to join rooms, send messages, and receive real-time updates.

## Features

- **Room-based chat**: Users can join specific rooms and chat with others in that room.
- **Real-time messaging**: Messages are sent and received instantly without needing to refresh the page.
- **Persistent messages**: The chat stores the latest messages in memory and sends them when users join a room.

## Tech Stack

- **Rust**: The backend is built with Rust for fast performance and memory safety.
- **socketioxide**: WebSocket library for handling real-time communication.
- **Axum**: A web framework for building APIs.
- **Tokio**: Asynchronous runtime for executing the server.
- **Serde**: Serialization and deserialization of data in JSON format.
- **Tracing**: For logging and tracking events.

## Installation

To get started with RustyChat, clone the repository and run the project with `cargo run`.

### Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Testing the Frontend with the Backend

If you want to test the frontend with the backend, you can clone the repository [https://github.com/dreamsofcode-io/chatly-web](https://github.com/dreamsofcode-io/chatly-web), install dependencies with `npm i`, and start the development server with `npm run dev`. The frontend will be available on port `:5173`, ready for testing.

## References

- [I never thought I'd use Socket.io ever again](https://www.youtube.com/watch?v=HEhhWL1oUTM)  
