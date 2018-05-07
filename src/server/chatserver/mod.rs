extern crate websocket;

mod data;
mod clienthandler;

use std::thread;
use std::sync::{Arc, Mutex};
use std::net::SocketAddrV4;
use std::sync::mpsc;

use self::clienthandler::ClientHandler;
use chatserver::data::chatmessage::ChatMessage;

type MsgSender = mpsc::Sender<ChatMessage>;

pub struct ChatServer;

impl ChatServer {
    pub fn launch() {
        let address =
            dotenv!("MIN_CHAT_SERVER_ADDRESS")
                .parse::<SocketAddrV4>()
                .expect("Could not parse server address");
        
        println!("Launching chat server on {:?}", address);

        let new_connections = websocket::sync::Server::bind(address).unwrap();

        let (cloneable_send_chan, recv_new_msgs) = mpsc::channel(); // client handler(s) -> chat server

        let chat_history: Arc<Mutex<Vec<ChatMessage>>> = Arc::new(Mutex::new(Vec::new()));
        let client_handlers: Arc<Mutex<Vec<MsgSender>>> = Arc::new(Mutex::new(Vec::new()));

        // Listens for new client connections, launches new client handlers.
        let conn_listener_client_handlers = client_handlers.clone();
        let conn_listener_chat_history = chat_history.clone();
        let h1 = thread::spawn(move || {
            for new_conn in new_connections.filter_map(Result::ok) {
                let client_ws = new_conn.accept().unwrap();

                println!("New client connected from {}", client_ws.peer_addr().unwrap());

                let (send_new_msgs, parent_recv_chan) = mpsc::channel(); // chat server -> client handler
                ClientHandler::launch(client_ws, parent_recv_chan, cloneable_send_chan.clone());

                for prev_msg in conn_listener_chat_history.lock().unwrap().iter() {
                    send_new_msgs.send(prev_msg.clone()).unwrap();
                }
                conn_listener_client_handlers.lock().unwrap().push(send_new_msgs);
            }
        });

        // Listens to client handlers, broadcasts new chat messages to all client handlers.
        let h2 = thread::spawn(move || {
            for new_msg in recv_new_msgs {
                println!("Got: '{:?}'", new_msg);
                for client_handler in client_handlers.lock().unwrap().iter() {
                    client_handler.send(new_msg.clone()).unwrap();
                }
                chat_history.lock().unwrap().push(new_msg);
            }
        });

        h1.join().unwrap();
        h2.join().unwrap();
    }
}