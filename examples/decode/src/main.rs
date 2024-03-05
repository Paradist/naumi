use std::io;
use naumi::nmacro::NaumiConvert;
use naumi::types::Convert;
use naumi::types::num::varint::VarInt;

#[derive(NaumiConvert, Debug)]
struct SomeStruct {
    d1: u32,
    d2: VarInt,
    d3: String
}

fn main() -> io::Result<()> {
    let mut encoded = [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33, 12, 6, 194, 129, 226, 99, 55].to_vec();

    println!("{:?}", SomeStruct::from_bytes(&mut encoded)?);
    Ok(())
}
