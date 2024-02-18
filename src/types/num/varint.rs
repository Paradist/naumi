use std::io;
use crate::types::Convert;
use crate::types::varint::{from_var_int_rev, to_var_int};

#[derive(Debug, Clone)]
pub struct VarInt(pub u32);

impl Convert for VarInt {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        let t = to_var_int(self.0);
        let mut t =t.0[..t.1 as usize].to_vec();
        t.reverse();
        tx.extend_from_slice(&t);
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let x = from_var_int_rev(rx)?;
        let _ = rx.drain(rx.len()-x.1..rx.len());
        Ok(
            Self(x.0)
        )
    }
}