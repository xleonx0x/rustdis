use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};


fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];
    
    let bytes_read = stream.read(&mut buf);

    stream.write(b"PING\n");
}

fn main() {
    let conn = TcpListener::bind("127.0.0.1:6767").unwrap();

    for stream in conn.incoming() {
        match stream {
            Ok(stream) => {
                println!("new connection received");
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(_) => todo!(),
        }
    }
}
