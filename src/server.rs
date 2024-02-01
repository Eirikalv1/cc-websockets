use simple_websockets::{Event, Responder, Message};
use std::collections::HashMap;

pub fn run() {
    let event_hub = simple_websockets::launch(1234)
    .expect("failed to listen on port 1234");

let mut clients: HashMap<u64, Responder> = HashMap::new();

loop {
    if let Some(event) = event_hub.next_event() {
        match event {
            Event::Connect(client_id, responder) => {
                println!("A client connected with id #{}", client_id);

                clients.insert(client_id, responder);
            },
            Event::Disconnect(client_id) => {
                println!("Client #{} disconnected.", client_id);

                clients.remove(&client_id);
            },
            Event::Message(client_id, mut message) => {               
                if let Message::Text(msg) = &message {
                    if msg == "nil" {
                        message = Message::Text("Command executed successfully".to_string());
                    }
                }
                println!("Received a message from client #{}: {:?}", client_id, message);

            },
        }
    };

    if let Some(responder) = clients.get(&0) {
        responder.send(Message::Text("turtle.down()".to_string()));
    }
    std::thread::sleep(std::time::Duration::from_millis(500));
}
}
