use oping::{Ping, PingResult, PingError};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::vec::Vec;

fn ping(host: &String) -> PingResult<()> {
    let mut ping = Ping::new();

    try!(ping.set_timeout(0.5));
    try!(ping.add_host(host.as_str()));

    let responses = try!(ping.send());

    for resp in responses {
        if resp.dropped > 0 {
            return Err(PingError::LibOpingError(String::from("No responses")));
        }
    }

    Ok(())
}

fn ping_list(hosts: &Vec<String>) -> HashMap<String, bool> {
    let mut status: HashMap<String, bool> = HashMap::new();
    
    for host in hosts {
        let active = match ping(&host) {
            Ok(_) => true,
            Err(_) => false,
        };

        status.insert(host.clone(), active);
    }

    status
}

pub fn start_ping_list(hosts: Vec<String>) -> Receiver<HashMap<String, bool>> {
    let (tx, rx): (Sender<HashMap<String, bool>>, Receiver<HashMap<String, bool>>) = mpsc::channel();
    let thread_hosts = hosts.clone();

    thread::spawn(move || {
        loop {
            tx.send(ping_list(&thread_hosts)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    rx
}
