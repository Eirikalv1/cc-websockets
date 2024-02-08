use macroquad::color::Color;
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
                        Self::message_event(&msg, blocks);
                    }
                }
            }
        };
    }

    pub fn send_message(&self, text: String) {
        if let Some(responder) = &self.client {
            responder.send(Message::Text(text));
        } else {
            log::error!("Cannot send message, no turtle connected!");
        }
    }

    fn message_event(msg: &str, blocks: &mut [Block]) {
        // Message protocol: "[Message type integer][Data]" e.g. '0minecraft:stone'
        // '0' -> Command executed successfully
        // '1[Error message]' -> Commaand failed
        // 2[Comma separated blocks][Block index] -> Geo Scanner data

        if msg.starts_with('0') {
            log::info!("Command executed successfully");
        } else if let Some(inner_msg) = msg.strip_prefix('1') {
            log::error!("Turtle does not understand the command! {}", inner_msg);
        } else if msg.starts_with('2') {
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
                    let hash: [u8; 16] = md5::compute(block.name.as_bytes()).into();

                    block.color = Color::new(
                        hash[0] as f32 / 255.,
                        hash[1] as f32 / 255.,
                        hash[2] as f32 / 255.,
                        1.0,
                    );
                } else {
                    block.name = "minecraft:air".to_string();
                }
            }
        }
    }
}

impl Default for Sockets {
    fn default() -> Self {
        Self::new()
    }
}
