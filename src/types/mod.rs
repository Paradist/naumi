use std::io;

pub mod num;
pub mod string;
pub mod iters;
pub mod varint;
pub mod other;

pub trait Convert {
    fn to_bytes(&self, tx: &mut Vec<u8>);
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> where Self: Sized;
}