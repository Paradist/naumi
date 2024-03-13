use std::io;
use std::io::{Error, ErrorKind};

use crate::types::Convert;
use crate::{impl_net_receive, types};

#[cfg(feature = "net")]
use std::io::{Read, Write};

#[cfg(feature = "net_async")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

impl Convert for bool {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        tx.push(*self as u8)
    }
    fn to_bytes_return(&self) -> Vec<u8> {
        vec![*self as u8]
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if let Some(u) = rx.pop() {
            Ok(u == 1)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![1, *self as u8])
    }

    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(
        &mut self,
        tx: &mut T,
    ) -> io::Result<()> {
        tx.write_all(&vec![1, *self as u8]).await
    }

    impl_net_receive!();
}
