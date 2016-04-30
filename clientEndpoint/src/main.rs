extern crate oping;

use oping::{Ping, PingResult};

fn ping(host: &str) -> PingResult<()> {
    let mut ping = Ping::new();

    println!("Setting timeout");
    try!(ping.set_timeout(5.0));
    println!("Adding host");
    try!(ping.add_host(host));

    println!("Sending");
    let responses = try!(ping.send());

    for resp in responses {
        if resp.dropped > 0 {
            println!("No responses from host {}", resp.hostname);
        } else {
            println!("Response received from {}", resp.hostname);
        }
    }

    Ok(())
}

fn main() {
    ping("4.2.2.2").expect("Error");
}
