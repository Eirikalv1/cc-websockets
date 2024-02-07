use axum::extract::Query;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, FmtSubscriber};

#[derive(Eq, PartialEq, Hash, Clone)]
struct TurtleId(String);
#[derive(Deserialize)]
struct Pagination {
    turtle_id: String,
}
impl Into<TurtleId> for Pagination {
    fn into(self) -> TurtleId {
        TurtleId(self.turtle_id)
    }
}

async fn turtle_manager(
    mut turtle_registry_channel: mpsc::UnboundedReceiver<(
        TurtleId,
        mpsc::UnboundedSender<Message>,
    )>,
    mut turtle_request_channel: mpsc::UnboundedReceiver<(
        TurtleId,
        oneshot::Sender<Option<mpsc::UnboundedSender<Message>>>,
    )>,
) {
    let mut turtle_registry: HashMap<TurtleId, mpsc::UnboundedSender<Message>> = HashMap::new();
    loop {
        tokio::select! {
            Some((turtle_id, sender)) = turtle_registry_channel.recv() => {turtle_registry.insert(turtle_id.clone(), sender.clone());},
            Some((turtle_id, sender)) = turtle_request_channel.recv() => {sender.send(turtle_registry.get(&turtle_id).cloned()).unwrap()}
        }
    }
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let (register_turtle, treg) = mpsc::unbounded_channel();
    let (get_turtle, treq) = mpsc::unbounded_channel();
    tokio::spawn(turtle_manager(treg, treq));
    let app_state = AppState {
        register_turtle,
        get_turtle,
    };
    // build our application with a single route
    let app = Router::new()
        .route(
            "/hw",
            get(|| async {
                println!("got request!");
                "Hello, World!"
            }),
        )
        .route("/ws", get(websocket_handler))
        .with_state(app_state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    register_turtle: mpsc::UnboundedSender<(TurtleId, mpsc::UnboundedSender<Message>)>,
    get_turtle: mpsc::UnboundedSender<(
        TurtleId,
        oneshot::Sender<Option<mpsc::UnboundedSender<Message>>>,
    )>,
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(pagination): Query<Pagination>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, app_state, pagination.into()))
}

async fn websocket(mut socket: WebSocket, app_state: AppState, turtle_id: TurtleId) {
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
