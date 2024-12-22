mod state;

use std::env;
use dotenv::dotenv;
use std::sync::Arc;
use axum::extract::State;
use axum::routing::{get};
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use tower::{ServiceBuilder};
use tower_http::cors::CorsLayer;
use tracing::{info};
use tracing_subscriber::FmtSubscriber;
use chrono::{Utc};
use tower_http::services::ServeDir;

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
    info!("New user connected");

    let store_clone = store.clone();
    socket.on("join", move |socket: SocketRef, Data(room): Data<String>| async move {
        info!("User {} connected to room: {:?}", format!("anon-{}", socket.id), room);

        let _ = socket.leave_all();
        let _ = socket.join(room.clone());
        let messages = store_clone.get(&room).await;
        let _ = socket.emit("messages", &Messages { messages });
    });

    let store_clone = store.clone();
    socket.on("message", move |socket: SocketRef, Data(data): Data<MessageIn>| async move {
        info!("Received message {:?} from user {}", data, format!("anon-{}", socket.id));

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
    dotenv().ok();
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let messages = state::MessageStore::default();

    let messages = Arc::new(messages);

    let (layer, io) = SocketIo::builder().with_state(messages.clone()).build_layer();

    let messages_for_handler = messages.clone();
    io.ns("/", move |socket| on_connect(socket, messages_for_handler.clone()));

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let app = axum::Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/hello", get(handler))
        .with_state(io)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer)
        );

    info!("Starting server on port :{}", port);

    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
