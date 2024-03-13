use std::io;
use std::io::{Error, ErrorKind};

pub fn to_var_int(i: u32) -> ([u8; 16], u8) {
    varint_simd::encode(i)
}

pub fn from_var_int(data: &[u8]) -> io::Result<(u32, usize)> {
    let mut value = 0u32;
    if data.len() < 5 {
        for i in 0..data.len() {
            value |= ((data[i] & 0x7F) as u32) << 7 * i;

            if (data[i] & 0x80) == 0 {
                return Ok((value, i + 1));
            }
        }
    } else {
        for i in 0..5 {
            value |= ((data[i] & 0x7F) as u32) << 7 * i;

            if (data[i] & 0x80) == 0 {
                return Ok((value, i + 1));
            }
        }
    }
    Err(Error::from(ErrorKind::InvalidData))
}

pub fn from_var_int_rev(data: &Vec<u8>) -> io::Result<(u32, usize)> {
    let mut value = 0u32;
    if data.len() < 5 {
        for i in 0..data.len() {
            value |= ((data[data.len() - 1 - i] & 0x7F) as u32) << 7 * i;

            if (data[i] & 0x80) == 0 {
                return Ok((value, i + 1));
            }
        }
    } else {
        for i in 0..5 {
            value |= ((data[data.len() - 1 - i] & 0x7F) as u32) << 7 * i;

            if (data[data.len() - 1 - i] & 0x80) == 0 {
                return Ok((value, i + 1));
            }
        }
    }
    Err(Error::from(ErrorKind::InvalidData))
}