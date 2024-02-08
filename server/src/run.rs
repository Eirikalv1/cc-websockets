use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::turtle_manager::{turtle_manager, TurtleId, TurtleRegistry};

#[derive(Deserialize)]
struct Pagination {
    turtle_id: String,
}
impl Into<TurtleId> for Pagination {
    fn into(self) -> TurtleId {
        TurtleId(self.turtle_id)
    }
}

pub async fn run() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let turtle_registry = TurtleRegistry::start();
    let app = Router::new()
        .route(
            "/hw",
            get(|| async {
                println!("got request!");
                "Hello, World!"
            }),
        )
        .route("/ws", get(websocket_handler))
        .with_state(turtle_registry);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(pagination): Query<Pagination>,
    State(app_state): State<TurtleRegistry>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, app_state, pagination.into()))
}

async fn websocket(mut socket: WebSocket, app_state: TurtleRegistry, turtle_id: TurtleId) {
    let (sender, mut receiver) = mpsc::unbounded_channel();
    app_state.register_turtle.send((turtle_id.clone(), sender));
    tracing::debug!("{} opened socket", turtle_id.0);
    loop {
        tokio::select! {
            socket_option = socket.recv() => match socket_option {
                Some(result) => match result {
                    Ok(msg) => {
                        let Message::Text(msg) = msg else {
                            panic!();
                        };
                        tracing::debug!("{} got socket message: {:?}", turtle_id.0, msg);
                        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
                        app_state.get_turtle.send((TurtleId(msg), oneshot_sender));
                        let a = oneshot_receiver.await.ok().flatten().unwrap();
                        let tutel = turtle_id.0.clone();
                        a.send(Message::Text(format!("{tutel} sendte deg en melding!!!")));
                    },
                    Err(err) => {socket.close(); return},
                },
                None => todo!(),
            },
            mpsc_option = receiver.recv() => match mpsc_option {
                Some(msg) => { tracing::debug!("{}, got mpsc message: {:?}", turtle_id.0, msg); socket.send(msg).await; },
                None => panic!()
            },
            else => break,
        }
    }
}
