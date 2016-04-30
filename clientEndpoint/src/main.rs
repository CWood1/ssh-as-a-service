extern crate oping;

use oping::{Ping, PingResult, PingError};
use std::collections::HashMap;
use std::vec::Vec;

fn ping(host: &str) -> PingResult<()> {
    let mut ping = Ping::new();

    try!(ping.set_timeout(5.0));
    try!(ping.add_host(host));

    let responses = try!(ping.send());

    for resp in responses {
        if resp.dropped > 0 {
            return Err(PingError::LibOpingError(String::from("No responses")));
        }
    }

    Ok(())
}

fn ping_list(hosts: Vec<&str>) -> HashMap<&str, bool> {
    let mut status: HashMap<&str, bool> = HashMap::new();
    
    for host in hosts {
        status.insert(host, 
                      match ping(host) {
                          Ok(_) => true,
                          Err(_) => false,
                      });
    }

    status
}

fn main() {
    let status = ping_list(vec!("10.0.0.1", "4.2.2.2", "example.com"));

    for (host, active) in status {
        if active {
            println!("Host {} is active", host);
        } else {
            println!("Host {} is inactive", host);
        }
    }
}
