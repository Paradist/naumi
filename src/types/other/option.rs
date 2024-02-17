use std::io;
use std::io::{Error, ErrorKind};

use crate::types::Convert;

impl<T: Convert> Convert for Option<T> {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        match self {
            None => {tx.push(0)}
            Some(some) => {
                tx.push(1);
                some.to_bytes(tx);
            }
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let x = rx.drain(0..1).as_slice()[0];
        Ok(
            match x {
                1 => { Self::from_bytes(rx)? }
                0 => { None }
                _ => { return Err(Error::from(ErrorKind::InvalidData)) }
            }
        )
    }
}