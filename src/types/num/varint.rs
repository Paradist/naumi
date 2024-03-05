use std::io;

use crate::types;
use crate::types::Convert;
use crate::types::varint::{from_var_int_rev, to_var_int};

#[cfg(feature = "net")]
use std::io::{Read, Write};

#[cfg(feature = "net_async")]
use tokio::io::{AsyncRead, AsyncWriteExt, AsyncReadExt, AsyncWrite};


///
/// A number with variable length.
///
/// The maximum value is u32::MAX
///
/// The maximum length is 5 bytes.
///
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "default_", derive(Default))]
#[cfg_attr(feature = "partial_eq", derive(PartialEq))]
pub struct VarInt(pub u32);

impl Convert for VarInt {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        let t = to_var_int(self.0);
        let mut t = t.0[..t.1 as usize].to_vec();
        t.reverse();
        tx.extend_from_slice(&t);
    }
    fn to_bytes_return(&self) -> Vec<u8> {
        let mut tx = vec![];
        &self.to_bytes(&mut tx);
        tx
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let x = from_var_int_rev(rx)?;
        rx.truncate(rx.len()-x.1);
        Ok(
            Self(x.0)
        )
    }

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        types::net::send(self, tx)
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        types::net::async_send(self, tx).await
    }

    #[cfg(feature = "net")]
    fn receive<T: Read>(rx: &mut T) -> io::Result<Self> {
        types::net::receive(rx)
    }

    #[cfg(feature = "net_async")]
    async fn async_receive<T: AsyncReadExt + Unpin + AsyncWrite>(rx: &mut T) -> io::Result<Self> {
        types::net::async_receive(rx).await
    }
}