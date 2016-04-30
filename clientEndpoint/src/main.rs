extern crate oping;
extern crate byteorder;

mod ping;
mod protocol;

fn main() {
    let rxstatus = ping::start_ping_list(vec!(String::from("10.0.0.1"), String::from("4.2.2.2"), String::from("example.com")));
    let stream = protocol::connect_to_server(String::from("10.8.0.1:5432"));

    protocol::handle_communication(stream, rxstatus);
}
