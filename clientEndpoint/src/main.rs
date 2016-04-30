extern crate oping;

mod ping;

fn main() {
    let status = ping::ping_list(vec!("10.0.0.1", "4.2.2.2", "example.com"));

    for (host, active) in status {
        if active {
            println!("Host {} is active", host);
        } else {
            println!("Host {} is inactive", host);
        }
    }
}
