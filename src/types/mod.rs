use std::io;

pub mod num;
pub mod string;
pub mod iters;
pub mod varint;
pub mod other;

#[cfg(feature = "net")]
use std::io::{Read, Write};

#[cfg(feature = "net_async")]
use tokio::io::{AsyncRead, AsyncWriteExt, AsyncReadExt, AsyncWrite};

#[cfg(any(feature = "net", feature = "net_async"))]
pub mod net;

pub trait Convert {
    ///
    /// Convert to bytes.
    ///
    fn to_bytes(&self, tx: &mut Vec<u8>);

    ///
    /// Convert to bytes.
    ///
    fn to_bytes_return(&self) -> Vec<u8>;

    ///
    /// Convert from bytes.
    ///
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> where Self: Sized;

    ///
    /// Send the data to the stream and clear the buffer.
    ///
    /// Use only this method of sending (Or async variant), because it safely sends data, specifying its length at the beginning, so that nothing is lost or stuck together.
    ///
    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()>;

    ///
    /// Send the data to the stream and clear the buffer.
    ///
    /// Use only this method of sending (Or syn variant), because it safely sends data, specifying its length at the beginning, so that nothing is lost or stuck together.
    ///
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()>;

    ///
    /// Get data from stream.
    ///
    /// Use only this method of sending (Or async variant), because it safely sends data, specifying its length at the beginning, so that nothing is lost or stuck together.
    ///
    #[cfg(feature = "net")]
    fn receive<T: Read>(rx: &mut T) -> io::Result<Self> where Self: Sized;

    ///
    /// Get data from stream.
    ///
    /// Use only this method of sending (Or syn variant), because it safely sends data, specifying its length at the beginning, so that nothing is lost or stuck together.
    ///
    #[cfg(feature = "net_async")]
    async fn async_receive<T: AsyncReadExt + Unpin + AsyncWrite>(rx: &mut T) -> io::Result<Self> where Self: Sized;

}