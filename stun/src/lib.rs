extern crate bytes;
extern crate tokio_core;

use bytes::{Buf, BufMut, BigEndian};
use std::io;
use std::net::SocketAddr;
use tokio_core::net::UdpCodec;

pub struct StunCodec;

pub const MAGIC_COOKIE: u32 = 0x2112A442;

pub enum MessageClass {
    Request = 0b00,
    Indication = 0b01,
    SuccessResponse = 0b10,
    ErrorResponse = 0b11,
}

impl MessageClass {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(MessageClass::Request),
            0b01 => Some(MessageClass::Indication),
            0b10 => Some(MessageClass::SuccessResponse),
            0b11 => Some(MessageClass::ErrorResponse),
            _ => None,
        }
    }
}

pub struct Message {
    class: MessageClass,
    method: u16,
    transaction_id: [u8; 12],
    attributes: Vec<Attribute>,
}

fn data_err(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

fn data_assert(b: bool, msg: &str) -> io::Result<()> {
    if !b {
        Err(data_err(msg))
    } else {
        Ok(())
    }
}

impl Message {
    // TODO: TryFrom when solidified
    fn from_bytes<T: Buf>(value: &mut T) -> io::Result<Self> {
        data_assert(value.remaining() >= 20, "Not a full message")?;

        // Stolen from https://github.com/sile/rustun
        let message_type = value.get_u16::<BigEndian>();
        data_assert(message_type >> 14 == 0, "Invalid header")?;
        let class = ((message_type >> 4) & 0b01) | ((message_type >> 7) & 0b10);
        let class = MessageClass::from_u8(class as u8).ok_or_else(|| data_err("Invalid class"))?;
        let method = (message_type & 0b0000_0000_1111) | ((message_type >> 1) & 0b0000_0111_0000) |
            ((message_type >> 2) & 0b1111_1000_0000);
        data_assert(method < 0x1000, "Invalid method")?;

        let magic_cookie = value.get_u32::<BigEndian>();
        data_assert(magic_cookie == MAGIC_COOKIE, "Invalid cookie")?;

        unimplemented!()
    }

    fn to_bytes<T: BufMut>(&self, target: T) {
        unimplemented!()
    }
}

pub enum Attribute {

}

impl UdpCodec for StunCodec {
    type In = (SocketAddr, Message);
    type Out = (SocketAddr, Message);

    fn decode(&mut self, src: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        let mut buf = io::Cursor::new(buf);
        Message::from_bytes(&mut buf).map(|msg| (*src, msg))
    }

    fn encode(&mut self, (addr, msg): Self::Out, buf: &mut Vec<u8>) -> SocketAddr {
        msg.to_bytes(buf);
        addr
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn some_test_here() {
        unimplemented!()
    }
}
