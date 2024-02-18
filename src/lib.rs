pub use nmacro;

pub mod types;
use types::Convert;
use types::varint::{from_var_int, to_var_int};

use std::io::{ErrorKind, Read, Write};

#[cfg(feature = "async")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub struct Coder(Vec<u8>);

impl Coder {
    /// Returns Coder.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    /// use naumi::nmacro::NaumiConvert;
    ///
    /// #[derive(NaumiConvert)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8
    /// }
    ///
    /// fn main() -> io::Result<()> {
    ///     let pixel = Pixel {
    ///         r: 12,
    ///         g: 12,
    ///         b: 12,
    ///     };
    ///
    ///     let mut coder = Coder::new();
    ///     coder.push(&pixel);
    ///
    ///     assert_eq!(coder.get_ref(), &vec![12,12,12]);
    ///
    ///     assert_eq!(Pixel::from_bytes(&mut coder.get_mut_ref())?, pixel);
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Self { Self(vec![]) }

    ///
    /// Clear buffer.
    ///
    pub fn clear(&mut self) {
        self.0.clear();
    }

    ///
    /// Add data to the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut coder = Coder::new();
    ///     coder.push(&0u16);
    ///     coder.push(&0u16);
    ///
    ///     assert_eq!(coder.get_ref(), &vec![0, 0, 0, 0]);
    ///     Ok(())
    /// }
    /// ```
    pub fn push<T: Convert>(&mut self, x: &T) {
        x.to_bytes(&mut self.0)
    }

    ///
    /// Get a ref to the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut coder = Coder::new();
    ///     coder.push(&0u16);
    ///     coder.push(&0u16);
    ///
    ///     assert_eq!(coder.get_ref(), &vec![0, 0, 0, 0]);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_ref(&self) -> &Vec<u8> {
        &self.0
    }

    ///
    /// Get a mut ref to the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut coder = Coder::new();
    ///     coder.push(&0u16);
    ///     coder.push(&0u16);
    ///
    ///     assert_eq!(coder.get_ref(), &mut vec![0, 0, 0, 0]);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_mut_ref(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }

    ///
    /// Send the data to the stream and clear the buffer.
    ///
    /// Use only this method of sending (Or async variant), because it safely sends data, specifying its length at the beginning, so that nothing is lost or stuck together.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    ///
    /// fn main() -> io::Result<()> {
    ///     use naumi::types::num::varint::VarInt;
    ///     let mut stream = TcpStream::connect("0.0.0.0:80")?;
    ///
    ///     let mut coder = Coder::new();
    ///
    ///     coder.push(&VarInt(12121212));
    ///     coder.send(&mut stream)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn send<T: Write>(&mut self, tx: &mut T) -> std::io::Result<()> {
        if self.0.len() > u32::MAX as usize {
            return Err(std::io::Error::from(ErrorKind::InvalidData))
        }
        let t = to_var_int(self.0.len() as u32);
        self.0.clear();
        tx.write_all(&t.0[..t.1 as usize])?;
        tx.write_all(&self.0)
    }

    ///
    /// Send the data to the stream and clear the buffer.
    ///
    /// Use only this method of sending (Or syn variant), because it safely sends data, specifying its length at the beginning, so that nothing is lost or stuck together.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    /// use naumi::types::num::varint::VarInt;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let mut stream = TcpStream::connect("0.0.0.0:80").await?;
    ///     let mut coder = Coder::new();
    ///
    ///     coder.push(&VarInt(12121212));
    ///     coder.send(&mut stream).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
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

    ///
    /// Get data from stream.
    ///
    /// Use only this method of sending (Or async variant), because it safely sends data, specifying its length at the beginning, so that nothing is lost or stuck together.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    /// use naumi::types::num::varint::VarInt;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut stream = TcpStream::connect("0.0.0.0:80")?;
    ///     let mut coder = Coder::new();
    ///
    ///     coder.receive(&mut stream)?;
    ///     let rx = &mut coder.get_mut_ref();
    ///
    ///     let varint = VarInt::from_bytes(rx)?;
    ///
    ///     Ok(())
    /// }
    /// ```
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

    ///
    /// Get data from stream.
    ///
    /// Use only this method of sending (Or syn variant), because it safely sends data, specifying its length at the beginning, so that nothing is lost or stuck together.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    /// use naumi::types::num::varint::VarInt;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let mut stream = TcpStream::connect("0.0.0.0:80").await?;
    ///     let mut coder = Coder::new();
    ///
    ///     coder.receive(&mut stream).await?;
    ///     let rx = &mut coder.get_mut_ref();
    ///
    ///     let varint = VarInt::from_bytes(rx)?;
    ///
    ///     Ok(())
    /// }
    /// ```
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
