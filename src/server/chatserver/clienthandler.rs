extern crate websocket;
extern crate serde_json;

use chatserver::websocket::OwnedMessage::Text;
use chatserver::data::chatmessage::ChatMessage;

use std::sync::mpsc;
use std::thread;
use std::net::TcpStream;

type WsClient = websocket::sync::Client<TcpStream>;

type MsgReceiver = mpsc::Receiver<ChatMessage>;
type MsgSender = mpsc::Sender<ChatMessage>;

pub struct ClientHandler;

impl ClientHandler {

    pub fn launch(client_ws: WsClient, parent_recv_chan: MsgReceiver, parent_send_chan: MsgSender) {
        let (mut client_recv_ws, mut client_send_ws) = client_ws.split().unwrap();

        thread::spawn(move || {
            for new_msg in client_recv_ws.incoming_messages() {
                match new_msg.unwrap() {
                    Text(txt) => {
                        let chat_message: ChatMessage = serde_json::from_str(&txt).unwrap();
                        parent_send_chan.send(chat_message).unwrap();
                    },
                    _ => break,
                }
            }
        });

        thread::spawn(move || {
            for new_msg in &parent_recv_chan {
                client_send_ws.send_message(&new_msg.into_message()).unwrap();
            }
        });
    }
}