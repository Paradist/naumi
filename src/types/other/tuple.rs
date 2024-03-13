use std::io;

use crate::types::Convert;
use crate::{impl_net, types};

#[cfg(feature = "net")]
use std::io::{Read, Write};

#[cfg(feature = "net_async")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

impl<A> Convert for (A,)
where
    A: Convert,
{
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        self.0.to_bytes(tx);
    }
    fn to_bytes_return(&self) -> Vec<u8> {
        let mut tx = Vec::new();
        self.0.to_bytes(&mut tx);
        tx
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        Ok((A::from_bytes(rx)?,))
    }

    impl_net!();
}

impl<A, B> Convert for (A, B)
where
    A: Convert,
    B: Convert,
{
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        self.1.to_bytes(tx);
        self.0.to_bytes(tx);
    }
    fn to_bytes_return(&self) -> Vec<u8> {
        let mut tx = Vec::new();
        self.1.to_bytes(&mut tx);
        self.0.to_bytes(&mut tx);
        tx
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        Ok((A::from_bytes(rx)?, B::from_bytes(rx)?))
    }

    impl_net!();
}

impl<A, B, C> Convert for (A, B, C)
where
    A: Convert,
    B: Convert,
    C: Convert,
{
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        self.2.to_bytes(tx);
        self.1.to_bytes(tx);
        self.0.to_bytes(tx);
    }
    fn to_bytes_return(&self) -> Vec<u8> {
        let mut tx = Vec::new();
        self.2.to_bytes(&mut tx);
        self.1.to_bytes(&mut tx);
        self.0.to_bytes(&mut tx);
        tx
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        Ok((A::from_bytes(rx)?, B::from_bytes(rx)?, C::from_bytes(rx)?))
    }

    impl_net!();
}

impl<A, B, C, D> Convert for (A, B, C, D)
where
    A: Convert,
    B: Convert,
    C: Convert,
    D: Convert,
{
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        self.3.to_bytes(tx);
        self.2.to_bytes(tx);
        self.1.to_bytes(tx);
        self.0.to_bytes(tx);
    }
    fn to_bytes_return(&self) -> Vec<u8> {
        let mut tx = Vec::new();
        self.3.to_bytes(&mut tx);
        self.2.to_bytes(&mut tx);
        self.1.to_bytes(&mut tx);
        self.0.to_bytes(&mut tx);
        tx
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        Ok((
            A::from_bytes(rx)?,
            B::from_bytes(rx)?,
            C::from_bytes(rx)?,
            D::from_bytes(rx)?,
        ))
    }

    impl_net!();
}

impl<A, B, C, D, E> Convert for (A, B, C, D, E)
where
    A: Convert,
    B: Convert,
    C: Convert,
    D: Convert,
    E: Convert,
{
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        self.4.to_bytes(tx);
        self.3.to_bytes(tx);
        self.2.to_bytes(tx);
        self.1.to_bytes(tx);
        self.0.to_bytes(tx);
    }
    fn to_bytes_return(&self) -> Vec<u8> {
        let mut tx = Vec::new();
        self.4.to_bytes(&mut tx);
        self.3.to_bytes(&mut tx);
        self.2.to_bytes(&mut tx);
        self.1.to_bytes(&mut tx);
        self.0.to_bytes(&mut tx);
        tx
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        Ok((
            A::from_bytes(rx)?,
            B::from_bytes(rx)?,
            C::from_bytes(rx)?,
            D::from_bytes(rx)?,
            E::from_bytes(rx)?,
        ))
    }

    impl_net!();
}

impl<A, B, C, D, E, F> Convert for (A, B, C, D, E, F)
where
    A: Convert,
    B: Convert,
    C: Convert,
    D: Convert,
    E: Convert,
    F: Convert,
{
    fn to_bytes(&self, tx: &mut Vec<u8>) {
        self.5.to_bytes(tx);
        self.4.to_bytes(tx);
        self.3.to_bytes(tx);
        self.2.to_bytes(tx);
        self.1.to_bytes(tx);
        self.0.to_bytes(tx);
    }
    fn to_bytes_return(&self) -> Vec<u8> {
        let mut tx = Vec::new();
        self.5.to_bytes(&mut tx);
        self.4.to_bytes(&mut tx);
        self.3.to_bytes(&mut tx);
        self.2.to_bytes(&mut tx);
        self.1.to_bytes(&mut tx);
        self.0.to_bytes(&mut tx);
        tx
    }
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> {
        Ok((
            A::from_bytes(rx)?,
            B::from_bytes(rx)?,
            C::from_bytes(rx)?,
            D::from_bytes(rx)?,
            E::from_bytes(rx)?,
            F::from_bytes(rx)?,
        ))
    }

    impl_net!();
}
