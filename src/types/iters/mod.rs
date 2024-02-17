use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io;
use std::io::{Error, ErrorKind};

use crate::types::Convert;
use crate::types::varint::{from_var_int, to_var_int};

#[derive(Debug)]
pub struct TinyVec<T: Convert>(pub Vec<T>);
#[derive(Debug)]
pub struct ShortVec<T: Convert>(pub Vec<T>);
#[derive(Debug)]
pub struct MediumVec<T: Convert>(pub Vec<T>);
#[derive(Debug)]
pub struct LongVec<T: Convert>(pub Vec<T>);

impl<T: Convert> Convert for TinyVec<T> {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > 255 {
            tx.push(255);
            for i in 0..25usize { self.0[i].to_bytes(tx) };
        } else {
            tx.push(self.0.len() as u8);
            for i in &self.0 {i.to_bytes(tx)};
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 1 { return Err(Error::new(ErrorKind::InvalidData, "")); }

        let size = u8::from_bytes(rx)?;
        let mut res = Self(Vec::new());
        for _ in 0..size {
            res.0.push(T::from_bytes(rx)?)
        }
        Ok(res)
    }
}

impl<T: Convert> Convert for ShortVec<T> {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > u16::MAX as usize {
            tx.extend_from_slice(&[255, 255]);
            for i in 0..u16::MAX as usize { self.0[i].to_bytes(tx) };
        } else {
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..2]);
            for i in &self.0 {i.to_bytes(tx)};
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 2 { return Err(Error::new(ErrorKind::InvalidData, "")); }

        let size = u16::from_bytes(rx)?;
        let mut res = Self(Vec::new());
        for _ in 0..size {
            res.0.push(T::from_bytes(rx)?)
        }
        Ok(res)
    }
}

impl<T: Convert> Convert for MediumVec<T> {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > u32::MAX as usize {
            tx.extend_from_slice(&[255, 255, 255, 255]);
            for i in 0..u32::MAX as usize { self.0[i].to_bytes(tx) }
        } else {
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..4]);
            for i in &self.0 {i.to_bytes(tx)};
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 4 { return Err(Error::new(ErrorKind::InvalidData, "")); }

        let size = u32::from_bytes(rx)?;
        let mut res = Self(Vec::new());
        for _ in 0..size {
            res.0.push(T::from_bytes(rx)?)
        }
        Ok(res)
    }
}

impl<T: Convert> Convert for LongVec<T> {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > u64::MAX as usize {
            tx.extend_from_slice(&[255, 255, 255, 255, 255, 255, 255, 255]);
            for i in 0..u64::MAX as usize { self.0[i].to_bytes(tx) }
        } else {
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..8]);
            for i in &self.0 {i.to_bytes(tx)};
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 8 { return Err(Error::from(ErrorKind::InvalidData)); }

        let size = u64::from_bytes(rx)?;
        let mut res = Self(Vec::new());
        for _ in 0..size {
            res.0.push(T::from_bytes(rx)?)
        }
        Ok(res)
    }
}
// Var Int
impl<T: Convert> Convert for Vec<T> {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.len() > u32::MAX as usize {
            let t = to_var_int(u32::MAX);
            tx.extend_from_slice(&t.0[..t.1 as usize]);
            for i in 0..u32::MAX as usize { self[i].to_bytes(tx) }
        } else {
            let t = to_var_int(self.len() as u32);
            tx.extend_from_slice(&t.0[..t.1 as usize]);
            for i in self {i.to_bytes(tx)};
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 1 { return Err(Error::from(ErrorKind::InvalidData)); }

        let size = from_var_int(&rx)?;
        let _ = rx.drain(0..size.1);

        let mut res = Self::new();
        for _ in 0..size.0 {
            res.push(T::from_bytes(rx)?)
        }
        Ok(res)
    }
}

// Var Int
impl<T: Convert + Eq + PartialEq + Hash> Convert for HashSet<T> {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.len() > u32::MAX as usize {
            let t = to_var_int(u32::MAX);
            tx.extend_from_slice(&t.0[..t.1 as usize]);
            let mut counter = 0u32;
            for i in self { i.to_bytes(tx); counter += 1; if counter == u32::MAX {return;}}
        } else {
            let t = to_var_int(self.len() as u32);
            tx.extend_from_slice(&t.0[..t.1 as usize]);
            for i in self {i.to_bytes(tx)};
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 1 { return Err(Error::from(ErrorKind::InvalidData)); }

        let size = from_var_int(&rx)?;
        let _ = rx.drain(0..size.1);

        let mut res = Self::new();
        for _ in 0..size.0 {
            res.insert(T::from_bytes(rx)?);
        }
        Ok(res)
    }
}

// Var Int
impl<T, J> Convert for HashMap<T, J>
where
    T: Convert + Eq + PartialEq + Hash,
    J: Convert + Eq + PartialEq + Hash
{
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.len() > u32::MAX as usize {
            let t = to_var_int(u32::MAX);
            tx.extend_from_slice(&t.0[..t.1 as usize]);
            let mut counter = 0u32;
            for i in self { i.0.to_bytes(tx); i.1.to_bytes(tx); counter += 1; if counter == u32::MAX {return;}}
        } else {
            let t = to_var_int(self.len() as u32);
            tx.extend_from_slice(&t.0[..t.1 as usize]);
            for i in self {i.0.to_bytes(tx); i.1.to_bytes(tx);};
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 1 { return Err(Error::from(ErrorKind::InvalidData)); }

        let size = from_var_int(&rx)?;
        let _ = rx.drain(0..size.1);

        let mut res = Self::new();
        for _ in 0..size.0*2 {
            res.insert(T::from_bytes(rx)?, J::from_bytes(rx)?);
        }
        Ok(res)
    }
}

