extern crate bytes;
extern crate tokio_core;

use bytes::BufMut;
use bytes::Bytes;
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

pub struct Message {
    class: MessageClass,
    method: u16,
    transaction_id: [u8; 12],
    attributes: Vec<Attribute>,
}

impl Message {
    // TODO: TryFrom when solidified
    fn from_bytes(value: Bytes) -> io::Result<Self> {
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
        Message::from_bytes(Bytes::from(buf)).map(|msg| (*src, msg))
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
