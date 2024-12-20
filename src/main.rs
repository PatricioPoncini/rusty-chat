mod state;

use std::sync::Arc;
use axum::extract::State;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use tower::{ServiceBuilder};
use tower_http::cors::CorsLayer;
use tracing::{info};
use tracing_subscriber::FmtSubscriber;
use chrono::{Utc};

#[derive(Debug, Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(Serialize)]
struct Messages {
    messages: Vec<state::Message>
}

async fn on_connect(socket: SocketRef, store: Arc<state::MessageStore>) {
    info!("Connecting to socket {:?}", socket.id);

    let store_clone = store.clone();
    socket.on("join", move |socket: SocketRef, Data(room): Data<String>| async move {
        info!("Connected to room: {:?}", room);

        let _ = socket.leave_all();
        let _ = socket.join(room.clone());
        let messages = store_clone.get(&room).await;
        let _ = socket.emit("messages", &Messages { messages });
    });

    let store_clone = store.clone();
    socket.on("message", move |socket: SocketRef, Data(data): Data<MessageIn>| async move {
        info!("Received message {:?}", data);

        let response = state::Message {
            text: data.text,
            user: format!("anon-{}", socket.id),
            date: Utc::now(),
        };

        store_clone.insert(&data.room, response.clone()).await;
        let _ = socket.within(data.room).emit("message", &response);
    });
}

// Able to send real-time events to websocket clients
// when handling HTTP events
async fn handler(State(io): State<SocketIo>) {
    let _ = io.emit("hello", "world");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let messages = state::MessageStore::default();

    let messages = Arc::new(messages);

    let (layer, io) = SocketIo::builder().with_state(messages.clone()).build_layer();

    let messages_for_handler = messages.clone();
    io.ns("/", move |socket| on_connect(socket, messages_for_handler.clone()));

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/hello", get(handler))
        .with_state(io)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer)
        );

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
