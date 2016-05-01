extern crate oping;
extern crate byteorder;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

mod ping;
mod protocol;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    
    let rxstatus = ping::start_ping_list(rx);
    let stream = protocol::connect_to_server(String::from("127.0.0.1:5432"));

    protocol::handle_communication(stream, rxstatus, tx);
}
