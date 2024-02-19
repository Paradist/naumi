use std::io;
use std::io::{Error, ErrorKind};
use crate::types::Convert;

impl Convert for bool {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.push(*self as u8)}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if let Some(u) = rx.pop() {
            Ok(u == 1)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}