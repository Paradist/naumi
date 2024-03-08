use std::{io, mem};
use std::io::{Error, ErrorKind};
use std::ops::{Add, Mul};

use crate::{impl_net_receive, types};
use crate::types::Convert;

#[cfg(feature = "net")]
use std::io::{Read, Write};

#[cfg(feature = "net_async")]
use tokio::io::{AsyncRead, AsyncWriteExt, AsyncReadExt, AsyncWrite};


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
    fn to_bytes_return(&self) -> Vec<u8> {vec![*self]}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if let Some(u) = rx.pop() {
            Ok(u)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![1, *self])
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![1, *self]).await
    }

    impl_net_receive!();
}
impl Convert for u16 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![2])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![2]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}
impl Convert for u32 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![4])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![4]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}
impl Convert for u64 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}
impl Convert for u128 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![16])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![16]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}

impl Convert for i8 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.push(*self as u8)}
    fn to_bytes_return(&self) -> Vec<u8> { vec![*self as u8] }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        if let Some(i) = rx.pop() {
            Ok(i as i8)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![1, *self as u8])
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![1, *self as u8]).await
    }

    impl_net_receive!();
}
impl Convert for i16 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![2])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![2]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}
impl Convert for i32 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![4])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![4]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}
impl Convert for i64 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}
impl Convert for i128 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![16])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![16]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}

impl Convert for usize {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&(*self as u64).to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {(*self as u64).to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {Ok(from_le_bytes::<u64>(rx)? as Self)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8])?;
        tx.write_all(&((*self as u64).to_le_bytes()))
    }

    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8]).await?;
        tx.write_all(&((*self as u64).to_le_bytes())).await
    }

    #[cfg(feature = "net")]
    fn receive<T: Read>(rx: &mut T) -> io::Result<Self> {
        Ok(types::net::receive::<u64, T>(rx)? as Self)
    }

    #[cfg(feature = "net_async")]
    async fn async_receive<T: AsyncReadExt + Unpin + AsyncWrite>(rx: &mut T) -> io::Result<Self> {
        Ok(types::net::async_receive::<u64, T>(rx).await? as Self)
    }
}
impl Convert for isize {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&(*self as i64).to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {(*self as i64).to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {Ok(from_le_bytes::<i64>(rx)? as Self)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8])?;
        tx.write_all(&((*self as i64).to_le_bytes()))
    }

    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8]).await?;
        tx.write_all(&((*self as i64).to_le_bytes())).await
    }

    #[cfg(feature = "net")]
    fn receive<T: Read>(rx: &mut T) -> io::Result<Self> {
        Ok(types::net::receive::<i64, T>(rx)? as Self)
    }

    #[cfg(feature = "net_async")]
    async fn async_receive<T: AsyncReadExt + Unpin + AsyncWrite>(rx: &mut T) -> io::Result<Self> {
        Ok(types::net::async_receive::<i64, T>(rx).await? as Self)
    }
}

impl Convert for f32 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![4])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![4]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}
impl Convert for f64 {
    fn to_bytes(&self, tx: &mut Vec<u8>) {tx.extend_from_slice(&self.to_le_bytes())}
    fn to_bytes_return(&self) -> Vec<u8> {self.to_le_bytes().to_vec()}
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {from_le_bytes(rx)}

    #[cfg(feature = "net")]
    fn send<T: Write>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8])?;
        tx.write_all(&self.to_le_bytes())
    }
    #[cfg(feature = "net_async")]
    async fn async_send<T: AsyncWriteExt + Unpin + AsyncRead>(&mut self, tx: &mut T) -> io::Result<()> {
        tx.write_all(&vec![8]).await?;
        tx.write_all(&self.to_le_bytes()).await
    }

    impl_net_receive!();
}
