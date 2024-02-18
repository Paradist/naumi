use std::io;
use std::os::unix::prelude::OsStrExt;

use crate::types::Convert;
use crate::types::varint::{from_var_int_rev, to_var_int};

///
/// String prefixed with u8.
///
/// Maximum length - u8::MAX
///
#[derive(Debug, Clone)]
pub struct TinyString(pub String);

///
/// String prefixed with u16.
///
/// Maximum length - u16::MAX
///
#[derive(Debug, Clone)]
pub struct ShortString(pub String);

///
/// String prefixed with u32.
///
/// Maximum length - u32::MAX
///
#[derive(Debug, Clone)]
pub struct MediumString(pub String);

///
/// String prefixed with u64.
///
/// Maximum length - u64::MAX
///
#[derive(Debug, Clone)]
pub struct LongString(pub String);

impl Convert for TinyString {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > 255 {
            tx.extend_from_slice(&self.0.as_bytes()[0..255]);
            tx.push(255);
        } else {
            tx.extend_from_slice(self.0.as_bytes());
            tx.push(self.0.len() as u8);
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = u8::from_bytes(rx)?;
        Ok(Self(String::from_utf8_lossy(rx.drain(rx.len()-size as usize ..rx.len()).as_slice()).to_string()))
    }
}

impl Convert for ShortString {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > u16::MAX as usize {
            tx.extend_from_slice(&self.0.as_bytes()[0..u16::MAX as usize]);
            tx.extend_from_slice(&[255, 255]);
        } else {
            tx.extend_from_slice(self.0.as_bytes());
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..2]);
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = u16::from_bytes(rx)?;
        Ok(Self(String::from_utf8_lossy(rx.drain(rx.len()-size as usize ..rx.len()).as_slice()).to_string()))
    }
}

impl Convert for MediumString {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > u32::MAX as usize {
            tx.extend_from_slice(&self.0.as_bytes()[0..u32::MAX as usize]);
            tx.extend_from_slice(&[255, 255, 255, 255]);
        } else {
            tx.extend_from_slice(self.0.as_bytes());
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..4]);
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = u32::from_bytes(rx)?;
        Ok(Self(String::from_utf8_lossy(rx.drain(rx.len()-size as usize ..rx.len()).as_slice()).to_string()))
    }
}

impl Convert for LongString {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > u64::MAX as usize {
            tx.extend_from_slice(&self.0.as_bytes()[0..u64::MAX as usize]);
            tx.extend_from_slice(&[255, 255, 255, 255, 255, 255, 255, 255]);
        } else {
            tx.extend_from_slice(self.0.as_bytes());
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..8]);
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = u64::from_bytes(rx)?;
        Ok(Self(String::from_utf8_lossy(rx.drain(rx.len()-size as usize ..rx.len()).as_slice()).to_string()))
    }
}

impl Convert for String {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.len() > u32::MAX as usize {
            tx.extend_from_slice(&self.as_bytes()[0..u32::MAX as usize]);
            let t = to_var_int(u32::MAX);
            let mut t =t.0[..t.1 as usize].to_vec();
            t.reverse();
            tx.extend_from_slice(&t);
        } else {
            tx.extend_from_slice(self.as_bytes());
            let t = to_var_int(self.len() as u32);
            let mut t =t.0[..t.1 as usize].to_vec();
            t.reverse();
            tx.extend_from_slice(&t);
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        let size = from_var_int_rev(&rx)?;
        let _ = rx.drain(rx.len()-size.1..rx.len());

        Ok(String::from_utf8_lossy(rx.drain(rx.len()-size.0 as usize..rx.len()).as_slice()).to_string())
    }
}
