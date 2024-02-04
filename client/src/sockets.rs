use simple_websockets::{Event, EventHub, Message, Responder};

use crate::objects::Block;

pub struct Sockets {
    pub event_hub: EventHub,
    pub client: Option<Responder>,
}

impl Sockets {
    pub fn new() -> Sockets {
        let event_hub = simple_websockets::launch(1234).expect("failed to listen on port 1234");
        let client = None;

        Self { event_hub, client }
    }

    pub fn process(&mut self, blocks: &mut [Block]) {
        if let Some(event) = self.event_hub.next_event() {
            match event {
                Event::Connect(_, responder) => {
                    log::info!("Turtle connected.");

                    self.client = Some(responder);
                }
                Event::Disconnect(_) => {
                    log::info!("Turtle disconnected.");

                    self.client = None;
                }
                Event::Message(_, msg_frame) => {
                    if let Message::Text(msg) = msg_frame.clone() {
                        if msg == "nil" {
                            log::info!("Command executed successfully");
                            
                        } else if msg.chars().next() == Some('[') {
                            let names = msg
                                .split("][")
                                .next()
                                .unwrap()
                                .trim_start_matches('[')
                                .trim_end_matches(']')
                                .split(',')
                                .map(|s| s.trim_matches('"').to_string())
                                .collect::<Vec<String>>();
                            let coords_linearized = msg
                                .split("][")
                                .nth(1)
                                .unwrap()
                                .trim_start_matches('[')
                                .trim_end_matches(']')
                                .split(',')
                                .map(|s| s.parse::<u16>().unwrap())
                                .collect::<Vec<u16>>();

                            assert!(
                                blocks.len() == coords_linearized.len(),
                                "turtle and client have different scan radiuses"
                            );

                            for (block_index, name_index) in coords_linearized.iter().enumerate() {
                                let block = &mut blocks[block_index];

                                block.coord = Block::delinearize(block_index as u16);
                                if *name_index > 0 {
                                    block.name = names[*name_index as usize - 1].to_owned();
                                } else {
                                    block.name = "minecraft:air".to_string();
                                }
                            }
                        } else {
                            log::error!("Turtle does not understand the command!");
                        }
                    }
                }
            }
        };
    }
}

impl Default for Sockets {
    fn default() -> Self {
        Self::new()
    }
}
