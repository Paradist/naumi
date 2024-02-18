use std::{io, mem};
use std::io::{Error, ErrorKind};
use std::ops::{Add, Mul};

use crate::types::Convert;

pub mod varint;

#[inline(always)]
fn from_le_bytes<T>(rx: &mut Vec<u8>) -> io::Result<T>
    where
        T: Sized,
{
    if rx.len() < mem::size_of::<T>() {
        return Err(io::Error::from(ErrorKind::UnexpectedEof));
    }

    let start = rx.len() - mem::size_of::<T>();
    let result;

    unsafe {
        let ptr = rx.as_ptr().add(start) as *const T;
        result = ptr.read_unaligned();
    }
    rx.truncate(start);
    Ok(result)
}

impl Convert for u8 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.push(*self)}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if let Some(u) = rx.pop() {
            Ok(u)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}
impl Convert for u16 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}
impl Convert for u32 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}
impl Convert for u64 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}
impl Convert for u128 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}

impl Convert for i8 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.push(*self as u8)}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if let Some(i) = rx.pop() {
            Ok(i as i8)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}
impl Convert for i16 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}
impl Convert for i32 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}
impl Convert for i64 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}
impl Convert for i128 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}

impl Convert for usize {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}
impl Convert for isize {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}

impl Convert for f32 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}
impl Convert for f64 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
}
