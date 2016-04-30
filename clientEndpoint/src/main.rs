extern crate oping;

mod ping;

fn main() {
    let rxstatus = ping::start_ping_list(vec!(String::from("10.0.0.1"), String::from("4.2.2.2"), String::from("example.com")));

    loop {
        let status = rxstatus.recv().unwrap();

        for (host, active) in status {
            if active {
                println!("Host {} is active", host);
            } else {
                println!("Host {} is inactive", host);
            }
        }
    }
}
