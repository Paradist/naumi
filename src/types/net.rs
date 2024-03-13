use std::io::ErrorKind;

use crate::types::varint::{from_var_int, to_var_int};
use crate::types::Convert;

#[cfg(feature = "net")]
use std::io::{Read, Write};

#[cfg(feature = "net_async")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[cfg(feature = "net")]
pub fn receive<T: Convert, J: Read>(rx: &mut J) -> std::io::Result<T> {
    let mut buf_1 = [0u8; 1];
    let mut buf_5 = [0u8; 5];
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
            break;
        }
    }
    T::from_bytes(&mut packet)
}

#[cfg(feature = "net_async")]
pub async fn async_receive<T: Convert, J: AsyncReadExt + Unpin + AsyncRead>(
    rx: &mut J,
) -> std::io::Result<T> {
    let mut buf = [0u8; 5];
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
            break;
        }
    }
    Ok(T::from_bytes(&mut packet)?)
}

#[cfg(feature = "net")]
pub fn send<T: Convert, J: Write>(data: &mut T, tx: &mut J) -> std::io::Result<()> {
    let encoded = data.to_bytes_return();
    if encoded.len() > u32::MAX as usize {
        return Err(std::io::Error::from(ErrorKind::InvalidData));
    }
    let t = to_var_int(encoded.len() as u32);
    tx.write_all(&t.0[..t.1 as usize])?;
    tx.write_all(&encoded)
}

#[cfg(feature = "net_async")]
pub async fn async_send<T: Convert, J: AsyncWriteExt + Unpin + AsyncWrite>(
    data: &mut T,
    tx: &mut J,
) -> std::io::Result<()> {
    let encoded = data.to_bytes_return();
    if encoded.len() > u32::MAX as usize {
        return Err(std::io::Error::from(ErrorKind::InvalidData));
    }
    let t = to_var_int(encoded.len() as u32);
    tx.write_all(&t.0[..t.1 as usize]).await?;
    tx.write_all(&encoded).await
}
