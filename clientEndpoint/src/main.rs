extern crate oping;

use oping::{Ping, PingResult, PingError};

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

fn main() {
    match ping("10.0.0.1") {
        Ok(_) => println!("Ping successful!"),
        Err(PingError::LibOpingError(e)) => println!("Error: {}", e),
        Err(_) => println!("Unspecified error occurred"),
    }
}
