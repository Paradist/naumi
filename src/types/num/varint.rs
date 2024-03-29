use std::io;

use crate::types::varint::{from_var_int_rev, to_var_int};
use crate::types::Convert;
use crate::{impl_net, types};

#[cfg(feature = "net")]
use std::io::{Read, Write};

#[cfg(feature = "net_async")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

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
        rx.truncate(rx.len() - x.1);
        Ok(Self(x.0))
    }

    impl_net!();
}
