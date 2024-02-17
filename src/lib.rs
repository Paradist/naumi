pub use nmacro;

pub mod types;
use types::Convert;
use types::varint::{from_var_int, to_var_int};

use std::io::{ErrorKind, Read, Write};

#[cfg(feature = "async")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub struct Coder(Vec<u8>);

impl Coder {
    pub fn new() -> Self { Self(vec![]) }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn push<T: Convert>(&mut self, x: &T) {
        x.to_bytes(&mut self.0)
    }

    pub fn get_ref(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn get_mut_ref(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }

    pub fn send<T: Write>(&mut self, tx: &mut T) -> std::io::Result<()> {
        if self.0.len() > u32::MAX as usize {
            return Err(std::io::Error::from(ErrorKind::InvalidData))
        }
        let t = to_var_int(self.0.len() as u32);
        self.0.clear();
        tx.write_all(&t.0[..t.1 as usize])?;
        tx.write_all(&self.0)
    }

    #[cfg(feature = "async")]
    pub async fn async_send<T: AsyncWriteExt>(&mut self, tx: &mut T) -> tokio::io::Result<()> {
        if self.0.len() > u32::MAX as usize {
            return Err(tokio::io::Error::from(ErrorKind::InvalidData))
        }
        let t = to_var_int(self.0.len() as u32);
        self.0.clear();
        tx.write_all(&t.0[..t.1 as usize]).await?;
        tx.write_all(&self.0).await
    }

    pub fn receive<T: Convert, J: Read>(&self, rx: &mut J) -> std::io::Result<T> {
        let mut buf_1 = [0u8;1];
        let mut buf_5 = [0u8;5];
        let mut chunk = Vec::new();
        let mut packet = Vec::with_capacity(32);

        for i in 0..5 {
            rx.read_exact(&mut buf_1)?;
            buf_5[i] = buf_1[0];
            if buf_5[i] & 0x80 == 0 {
                if i == 0 {
                    chunk = vec![0u8; buf_5[i] as usize];
                    rx.read_exact(&mut chunk)?;
                    packet.push(buf_5[i]);
                    packet.extend(chunk);

                } else {
                    chunk = vec![0u8; from_var_int(&buf_5[0..=i])?.0 as usize];
                    rx.read_exact(&mut chunk)?;
                    packet.extend(&buf_5[0..=i]);
                    packet.extend(chunk);
                }
                break
            }
        }
        T::from_bytes(&mut packet)
    }
    #[cfg(feature = "async")]
    pub async fn async_receive<T: Convert, J: AsyncReadExt>(&self, rx: &mut J) -> tokio::io::Result<T> {
        let mut buf = [0u8;5];
        let mut chunk = Vec::new();
        let mut packet = Vec::with_capacity(32);

        for i in 0..5 {
            buf[i] = rx.read_u8().await?;
            if buf[i] & 0x80 == 0 {
                if i == 0 {
                    chunk = vec![0u8; buf[i] as usize];
                    rx.read_exact(&mut chunk).await?;
                    packet.push(buf[i]);
                    packet.extend(chunk);

                } else {
                    chunk = vec![0u8; from_var_int(&buf[0..=i])?.0 as usize];
                    rx.read_exact(&mut chunk).await?;
                    packet.extend(&buf[0..=i]);
                    packet.extend(chunk);
                }
                break
            }
        }
        Ok(T::from_bytes(&mut packet)?)
    }
}
