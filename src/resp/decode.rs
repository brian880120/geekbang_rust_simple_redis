use crate::{RespDecode, RespError, RespFrame, SimpleError, SimpleString};
use bytes::BytesMut;

impl RespDecode for RespFrame {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => {
                todo!()
            }
            _ => todo!(),
        }
    }
    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for SimpleString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if buf.len() < 3 {
            return Err(RespError::NotComplete)
        }

        if !buf.starts_with(b"+") {
            return Err(RespError::InvalidFrameType(format!(
                "expect: SimpleString(+), got: {:?}",
                buf
            )));
        }

        // search for "\r\n"
        let mut end = 0;
        for i in 0..buf.len() - 1 {
            if buf[i] == b'\r' && buf[i + 1] == b'\n' {
                end = i;
                break;
            }
        }

        if end == 0 {
            return Err(RespError::NotComplete);
        }

        // split the buffer
        let data = buf.split_to(end + 2);
        let s = String::from_utf8_lossy(&data[1..end]);
        Ok(SimpleString::new(s.to_string()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        todo!()
    }
}

// impl RespDecode for SimpleError {
//     fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {

//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use bytes::BufMut;

    #[test]
    fn test_simple_string_decode() -> Result<()>{
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"+OK\r\n");

        let frame = SimpleString::decode(&mut buf)?;
        assert_eq!(frame, SimpleString::new("OK".to_string()));

        buf.extend_from_slice(b"+hello\r");

        let ret = SimpleString::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.put_u8(b'\n');
        let frame = SimpleString::decode(&mut buf)?;
        assert_eq!(frame, SimpleString::new("hello".to_string()));
        Ok(())
    }
}
