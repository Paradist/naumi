use std::io;
use std::io::{Error, ErrorKind};
use std::mem::size_of;
use std::ops::{Add, Mul};

use crate::types::Convert;

pub mod varint;

fn from_le_bytes<T>(rx: &mut Vec<u8>) -> io::Result<T>
where
    T: Default + Add + Mul
{
    let size = size_of::<T>();
    let bytes: Vec<u8> = rx.drain(0..size).collect();
    let mut value: T = Default::default();

    if size == std::mem::size_of_val(&value) {
        let bytes_ptr = &mut value as *mut T as *mut u8;
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), bytes_ptr, size);
        }
    } else {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    Ok(value)
}

impl Convert for u8 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.push(*self)}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
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
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}
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
