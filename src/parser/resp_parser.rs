enum RedisValue<'a> {
    String(&'a [u8]),
    Int(i64)
}

fn word(buf: &[u8], pos: usize) -> Option<(usize, &[u8])> {
    if pos >= buf.len() {
        return None;
    }
    
    match buf[pos..].iter().position(|&b| b == b'\r') {
        Some(ind) => {
            Some((pos + ind + 2, &buf[pos..pos + ind]))
        }
        None => None,
    }
}

fn int(buf: &[u8], pos: usize) -> Option<(usize, i64)> {
    match word(buf, pos) {
        Some((pos, word)) => {
            Some((pos, str::from_utf8(word).unwrap().parse::<i64>().unwrap()))
        },
        None => None,
    }
}

fn simple_string(buf: &[u8], pos: usize) -> Option<RedisValue> {
    match word(buf, pos) {
        Some((pos, word)) => Some(RedisValue::String(word)),
        None => None,
    }
}

fn resp_int(buf: &[u8], pos: usize) -> Option<RedisValue> {
    match int(buf, pos) {
        Some((pos, int)) => Some(RedisValue::Int(int)),
        None => None,
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
