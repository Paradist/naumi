use std::io;
use std::os::unix::prelude::OsStrExt;

use crate::types::Convert;
use crate::types::varint::{from_var_int, to_var_int};

#[derive(Debug, Clone)]
pub struct TinyString(pub String);
#[derive(Debug, Clone)]
pub struct ShortString(pub String);
#[derive(Debug, Clone)]
pub struct MediumString(pub String);
#[derive(Debug, Clone)]
pub struct LongString(pub String);

impl Convert for TinyString {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > 255 {
            tx.push(255);
            tx.extend_from_slice(&self.0.as_bytes()[0..255]);
        } else {
            tx.push(self.0.len() as u8);
            tx.extend_from_slice(self.0.as_bytes());
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = u8::from_bytes(rx)?;
        Ok(Self(String::from_utf8_lossy(rx.drain(0..size as usize).as_slice()).to_string()))
    }
}

impl Convert for ShortString {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > u16::MAX as usize {
            tx.extend_from_slice(&[255, 255]);
            tx.extend_from_slice(&self.0.as_bytes()[0..u16::MAX as usize]);
        } else {
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..2]);
            tx.extend_from_slice(self.0.as_bytes());
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = u16::from_bytes(rx)?;
        Ok(Self(String::from_utf8_lossy(rx.drain(0..size as usize).as_slice()).to_string()))
    }
}

impl Convert for MediumString {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > u32::MAX as usize {
            tx.extend_from_slice(&[255, 255, 255, 255]);
            tx.extend_from_slice(&self.0.as_bytes()[0..u32::MAX as usize]);
        } else {
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..4]);
            tx.extend_from_slice(self.0.as_bytes());
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = u32::from_bytes(rx)?;
        Ok(Self(String::from_utf8_lossy(rx.drain(0..size as usize).as_slice()).to_string()))
    }
}

impl Convert for LongString {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > u64::MAX as usize {
            tx.extend_from_slice(&[255, 255, 255, 255, 255, 255, 255, 255]);
            tx.extend_from_slice(&self.0.as_bytes()[0..u64::MAX as usize]);
        } else {
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..8]);
            tx.extend_from_slice(self.0.as_bytes());
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = u64::from_bytes(rx)?;
        Ok(Self(String::from_utf8_lossy(rx.drain(0..size as usize).as_slice()).to_string()))
    }
}

impl Convert for String {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.len() > u32::MAX as usize {
            tx.extend_from_slice(&[255, 255, 255, 255, 255]);
            tx.extend_from_slice(&self.as_bytes()[0..u32::MAX as usize]);
        } else {
            let t = to_var_int(self.len() as u32);
            tx.extend_from_slice(&t.0[..t.1 as usize]);
            tx.extend_from_slice(self.as_bytes());
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = from_var_int(&rx[0..5])?;
        let _ = rx.drain(0..size.1);

        Ok(String::from_utf8_lossy(rx.drain(0..size.0 as usize).as_slice()).to_string())
    }
}
