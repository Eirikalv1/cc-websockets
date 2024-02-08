use axum::extract::ws::Message;
use std::collections::HashMap;
use std::future::Future;
use tokio::sync::{mpsc, oneshot};

#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct TurtleId(pub String);

pub(crate) async fn turtle_manager(
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

#[derive(Clone)]
pub(crate) struct TurtleRegistry {
    pub register_turtle: mpsc::UnboundedSender<(TurtleId, mpsc::UnboundedSender<Message>)>,
    pub get_turtle: mpsc::UnboundedSender<(
        TurtleId,
        oneshot::Sender<Option<mpsc::UnboundedSender<Message>>>,
    )>,
}

impl TurtleRegistry {
    pub(crate) fn start() -> TurtleRegistry {
        let (register_turtle, treg) = mpsc::unbounded_channel();
        let (get_turtle, treq) = mpsc::unbounded_channel();
        let app_state = TurtleRegistry {
            register_turtle,
            get_turtle,
        };
        tokio::spawn(turtle_manager(treg, treq));
        app_state
    }
}
