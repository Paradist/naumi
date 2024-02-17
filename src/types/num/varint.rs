use std::io;
use crate::types::Convert;
use crate::types::varint::{from_var_int, to_var_int};

#[derive(Debug, Clone)]
pub struct VarInt(pub u32);

impl Convert for VarInt {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        let t = to_var_int(self.0);
        tx.extend_from_slice(&t.0[..t.1 as usize])
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let x = from_var_int(rx)?;
        let _ = rx.drain(0..x.1);
        Ok(
            Self(x.0)
        )
    }
}