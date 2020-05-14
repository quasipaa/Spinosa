//! Protocol codec, which implements TcpSocket's streaming data encoding and decoding.
//!
//! All codec instances must implement the `Codec` and` Default` traits.
//! Provide an abstract interface for the TcpSocket instance to use.

pub mod rtmp;

use bytes::{Bytes, BytesMut};

/// Data packets returned by the codec
pub enum Packet {
    /// Tcp message
    Tcp(Bytes),
    /// Udp message, including flag
    Udp(Bytes, u8),
}

pub trait Codec {
    /// Parse TcpSocket data
    /// 
    /// Process the data and return the response packet,
    /// The response is divided into Tcp, Udp, Tcp data is sent to the peer,
    /// Udp data will be sent to the business backend for further processing.
    fn parse(&mut self, buffer: &mut BytesMut) -> Vec<Packet>;
}
