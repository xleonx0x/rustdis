use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn word(buf: &[u8], pos: usize) -> Option<(usize, &[u8])> {
    if pos >= buf.len() {
        return None;
    }
    
    match buf[pos..].iter().position(|b| *b == b'\r') {
        Some(ind) => {
            Some((pos + ind + 2, &buf[pos..pos + ind]))
        }
        None => None,
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word() {
        let buf = b"hello world\r\nbello\r\nhehehe\r\n";
        assert_eq!(word(buf, 0), Some((13, &b"hello world"[..])));
        assert_eq!(word(buf, 13), Some((20, &b"bello"[..])));
        assert_eq!(word(buf, 20), Some((28, &b"hehehe"[..])));
        assert_eq!(word(buf, 28), None);
    }
}
