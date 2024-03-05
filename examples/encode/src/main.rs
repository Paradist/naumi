use naumi::nmacro::NaumiConvert;
use naumi::types::Convert;
use naumi::types::num::varint::VarInt;

#[derive(NaumiConvert, Debug)]
struct SomeStruct {
    d1: u32,
    d2: VarInt,
    d3: String
}

fn main() {
    let some_struct = SomeStruct {
        d1: 929292929,
        d2: VarInt(834),
        d3: "Hello world!".to_string()
    };


    {
        println!("{:?}", some_struct.to_bytes_return());
    }

    {
        let mut encoded = vec![];
        some_struct.to_bytes(&mut encoded);
        println!("{:?}", encoded)
    }
}
