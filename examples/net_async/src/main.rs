#![feature(async_fn_in_trait)]

use std::io;
use naumi::nmacro::NaumiConvert;
use naumi::types::Convert;
use naumi::types::num::varint::VarInt;

use tokio::net::TcpStream;

#[derive(NaumiConvert, Debug)]
struct SomeStruct {
    d1: u32,
    d2: VarInt,
    d3: String
}

#[tokio::main]
async fn main() -> io::Result<()>{
    let mut some_struct = SomeStruct {
        d1: 929292929,
        d2: VarInt(834),
        d3: "Hello world!".to_string()
    };

    // send
    {
        let mut stream = TcpStream::connect("0.0.0.0:80").await?;
        some_struct.async_send(&mut stream).await?;
    }


    // receive
    {
        let mut stream = TcpStream::connect("0.0.0.0:80").await?;
        let some_struct = SomeStruct::async_receive(&mut stream).await?;
    }

    Ok(())
}
