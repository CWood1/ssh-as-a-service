use std::net::TcpStream;
use std::time::Duration;
use std::collections::HashMap;
use std::u16;
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::sync::mpsc::{Sender, Receiver};
use std::io::{Read, Write};
use std::io::Cursor;

pub fn connect_to_server(server: String) -> TcpStream {
    let stream = TcpStream::connect(server.as_str()).unwrap();

    stream.set_read_timeout(Some(Duration::from_millis(50))).unwrap();
    stream.set_write_timeout(Some(Duration::from_millis(50))).unwrap();

    stream
}

fn handle_pkt(mut stream: &TcpStream, host_status: HashMap<String, bool>, header: [u8; 4], host_list: &Sender<String>) {
    match header[0] {
        1 => {
            let mut rdr = Cursor::new(header);
            rdr.set_position(2);

            let length: u16 = rdr.read_u16::<LittleEndian>().unwrap();
            
            let mut payload: Vec<u8> = vec![0; length as usize];
            stream.read(&mut payload).unwrap();

            let host = String::from_utf8(payload).unwrap();

            host_list.send(host).unwrap();
        },
        2 => {
            let mut octets: Vec<u8> = vec![0, 0, 3, 0];
            
            for (host, status) in host_status {
                octets.extend(host.into_bytes());
                octets.push(if status { 1 } else { 0 });
            }

            let octet_length = octets.len() as u32 - 4;

            // TODO: At some point, this should spill over into two payloads. Consider using a chain flag in the last byte of the
            // header
            if octet_length > u16::max_value() as u32 {
                println!("Payload size too large.");
                return;
            }

            let mut length: Vec<u8> = vec![0, 0];
            length.write_u16::<LittleEndian>(octet_length as u16).unwrap();

            octets[0] = length[0];
            octets[1] = length[1];

            stream.write(&octets).unwrap();
        },
        _ => return,
    }
}

pub fn handle_communication(mut stream: TcpStream, host_status: Receiver<HashMap<String, bool>>, host_list: Sender<String>) {
    loop {
        let status = host_status.recv().unwrap();
        let mut header: [u8; 4] = [0; 4];

        match stream.read(&mut header) {
            Ok(4) => handle_pkt(&stream, status, header, &host_list),
            Ok(_) => continue,
            Err(_) => continue,
        }
    }
}
