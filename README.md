# RustyChat

RustyChat is a simple, real-time chat application built with **Rust** using the **socketioxide** library for WebSocket communication. It allows users to join rooms, send messages, and receive real-time updates.

## Features

- **Room-based chat**: Users can join specific rooms and chat with others in that room.
- **Real-time messaging**: Messages are sent and received instantly without needing to refresh the page.
- **Persistent messages**: The chat stores the latest messages in memory and sends them when users join a room.

## Tech Stack

[![Tech Stack](https://skillicons.dev/icons?i=rust,html,css,js)](https://skillicons.dev)

## Installation

To get started with RustyChat, clone the repository, create a `.env` file with the `PORT` variable, and run the project using `cargo run`.

### Example of .env file
```bash
PORT=3000 # any port you want
```

### Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Testing the Frontend with the Backend

You can access the static test frontend inspired by the legend of '[The Knights of the Round Table](https://en.wikipedia.org/wiki/Knights_of_the_Round_Table)' by navigating to `http://localhost:3000` or the port you’ve configured in your `.env` file.

## References

- [I never thought I'd use Socket.io ever again](https://www.youtube.com/watch?v=HEhhWL1oUTM)  
