use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io;
use std::io::{Error, ErrorKind};

use crate::types::Convert;
use crate::types::varint::{from_var_int_rev, to_var_int};


///
/// Vector prefixed with u8.
///
/// Maximum length - u8::MAX
///
#[derive(Debug)]
pub struct TinyVec<T: Convert>(pub Vec<T>);

///
/// Vector prefixed with u16.
///
/// Maximum length - u16::MAX
///
#[derive(Debug)]
pub struct ShortVec<T: Convert>(pub Vec<T>);

///
/// Vector prefixed with u32.
///
/// Maximum length - u32::MAX
///
#[derive(Debug)]
pub struct MediumVec<T: Convert>(pub Vec<T>);

///
/// Vector prefixed with u64.
///
/// Maximum length - u64::MAX
///
#[derive(Debug)]
pub struct LongVec<T: Convert>(pub Vec<T>);

impl<T: Convert> Convert for TinyVec<T> {
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        if self.0.len() > 255 {
            for i in 0..255usize { self.0[i].to_bytes(tx) };
            tx.push(255);
        } else {
            for i in &self.0 {i.to_bytes(tx)};
            tx.push(self.0.len() as u8);
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
            for i in 0..u16::MAX as usize { self.0[i].to_bytes(tx) };
            tx.extend_from_slice(&[255, 255]);
        } else {
            for i in &self.0 {i.to_bytes(tx)};
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..2]);
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
            for i in 0..u32::MAX as usize { self.0[i].to_bytes(tx) }
            tx.extend_from_slice(&[255, 255, 255, 255]);
        } else {
            for i in &self.0 {i.to_bytes(tx)};
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..4]);
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
            for i in 0..u64::MAX as usize { self.0[i].to_bytes(tx) }
            tx.extend_from_slice(&[255, 255, 255, 255, 255, 255, 255, 255]);
        } else {
            for i in &self.0 {i.to_bytes(tx)};
            tx.extend_from_slice(&self.0.len().to_le_bytes()[0..8]);
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
            for i in 0..u32::MAX as usize { self[i].to_bytes(tx) }
            let t = to_var_int(u32::MAX);
            let mut t =t.0[..t.1 as usize].to_vec();
            t.reverse();
            tx.extend_from_slice(&t);
        } else {
            for i in self {i.to_bytes(tx)};
            let t = to_var_int(self.len() as u32);
            let mut t =t.0[..t.1 as usize].to_vec();
            t.reverse();
            tx.extend_from_slice(&t);
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 1 { return Err(Error::from(ErrorKind::InvalidData)); }

        let size = from_var_int_rev(&rx)?;
        let _ = rx.drain(rx.len()-size.1..rx.len());

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
            let mut counter = 0u32;
            for i in self { i.to_bytes(tx); counter += 1; if counter == u32::MAX {return;}}
            let t = to_var_int(u32::MAX);
            let mut t =t.0[..t.1 as usize].to_vec();
            t.reverse();
            tx.extend_from_slice(&t);
        } else {
            for i in self {i.to_bytes(tx)};
            let t = to_var_int(self.len() as u32);
            let mut t =t.0[..t.1 as usize].to_vec();
            t.reverse();
            tx.extend_from_slice(&t);
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 1 { return Err(Error::from(ErrorKind::InvalidData)); }

        let size = from_var_int_rev(&rx)?;
        let _ = rx.drain(rx.len()-size.1..rx.len());

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
            let mut counter = 0u32;
            for i in self { i.0.to_bytes(tx); i.1.to_bytes(tx); counter += 1; if counter == u32::MAX {return;}}
            let t = to_var_int(u32::MAX);
            let mut t =t.0[..t.1 as usize].to_vec();
            t.reverse();
            tx.extend_from_slice(&t);
        } else {
            for i in self {i.0.to_bytes(tx); i.1.to_bytes(tx);};
            let t = to_var_int(self.len() as u32);
            let mut t =t.0[..t.1 as usize].to_vec();
            t.reverse();
            tx.extend_from_slice(&t);
        }
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if rx.len() < 1 { return Err(Error::from(ErrorKind::InvalidData)); }

        let size = from_var_int_rev(&rx)?;
        let _ = rx.drain(rx.len()-size.1..rx.len());

        let mut res = Self::new();
        for _ in 0..size.0 {
            let j = J::from_bytes(rx)?;
            res.insert(T::from_bytes(rx)?, j);
        }
        Ok(res)
    }
}

