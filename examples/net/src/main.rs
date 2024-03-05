use std::io;
use std::net::TcpStream;
use naumi::nmacro::NaumiConvert;
use naumi::types::Convert;
use naumi::types::num::varint::VarInt;

#[derive(NaumiConvert, Debug)]
struct SomeStruct {
    d1: u32,
    d2: VarInt,
    d3: String
}

fn main() -> io::Result<()>{
    let mut some_struct = SomeStruct {
        d1: 929292929,
        d2: VarInt(834),
        d3: "Hello world!".to_string()
    };

    // send
    {
        let mut stream = TcpStream::connect("0.0.0.0:80")?;
        some_struct.send(&mut stream)?;
    }


    // receive
    {
        let mut stream = TcpStream::connect("0.0.0.0:80")?;
        let some_struct = SomeStruct::receive(&mut stream)?;
    }

    Ok(())
}
