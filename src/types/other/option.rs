use std::io;
use std::io::{Error, ErrorKind};

use crate::types::Convert;
use crate::{impl_net, types};

#[cfg(feature = "net")]
use std::io::{Read, Write};

#[cfg(feature = "net_async")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

impl<T: Convert> Convert for Option<T> {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        match self {
            None => tx.push(0),
            Some(some) => {
                some.to_bytes(tx);
                tx.push(1);
            }
        }
    }
    fn to_bytes_return(&self) -> Vec<u8> {
        let mut tx = vec![];
        &self.to_bytes(&mut tx);
        tx
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let x = rx.split_off(rx.len() - 1)[0];
        Ok(match x {
            1 => Some(T::from_bytes(rx)?),
            0 => None,
            _ => return Err(Error::from(ErrorKind::InvalidData)),
        })
    }

    impl_net!();
}
