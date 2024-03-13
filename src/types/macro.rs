use tokio::io::{AsyncRead, AsyncWrite};
#[macro_export]
macro_rules! impl_net {
    () => {
        #[cfg(feature = "net")]
        fn send<_T: Write>(&mut self, tx: &mut _T) -> io::Result<()> {
            types::net::send(self, tx)
        }
        #[cfg(feature = "net_async")]
        async fn async_send<_T: AsyncWriteExt + Unpin + AsyncWrite>(
            &mut self,
            tx: &mut _T,
        ) -> io::Result<()> {
            types::net::async_send(self, tx).await
        }

        #[cfg(feature = "net")]
        fn receive<_T: Read>(rx: &mut _T) -> io::Result<Self> {
            types::net::receive(rx)
        }

        #[cfg(feature = "net_async")]
        async fn async_receive<_T: AsyncReadExt + Unpin + AsyncRead>(
            rx: &mut _T,
        ) -> io::Result<Self> {
            types::net::async_receive(rx).await
        }
    };
}

#[macro_export]
macro_rules! impl_net_receive {
    () => {
        #[cfg(feature = "net")]
        fn receive<_T: Read>(rx: &mut _T) -> io::Result<Self> {
            types::net::receive(rx)
        }

        #[cfg(feature = "net_async")]
        async fn async_receive<_T: AsyncReadExt + Unpin + AsyncRead>(
            rx: &mut _T,
        ) -> io::Result<Self> {
            types::net::async_receive(rx).await
        }
    };
}
