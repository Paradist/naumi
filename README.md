<h1 align=center>Naumi</h1>
<p align=center>Lightweight and fast data transfer protocol. Fast de/serialization and tiny size!</p>

## Why?


Naumi is an innovative data format designed to be a complete alternative to JSON and ProtoBuf, while providing greater efficiency and flexibility. It is designed to meet modern data processing requirements, offering compact serialization, improved read and write speeds, and support for complex data structures and types, making it an ideal choice for a wide range of applications, from webAPIs to real-time messaging and configuration files.

## 📝 To Do (before the 1.0 release)

- [ ] Describe the functions with examples.
- [ ] Data compression using zstd
- [ ] TLS
- [ ] More speed and optimization!
- [ ] More supported data types:
  - [ ] Let me know on the [issues page](https://github.com/Paradist/naumi/issues).


## Currently supported types

 **Numbers:** u8, u16, u32, u64, usize, u128, f32, f64, VarInt // and signed
 
 **Strings:** String(VarInt), TinyString(u8), ShortString(u16), MediumString(u32), LongString(u64) 
 
 **Iters:** Vec(VarInt), TinyVec(u8), ShortVec(u16), MediumVec(u32), LongVec(u64), HashSet(VarInt), HashMap(VarInt)
 
 **Custom:** Structs(Named/Unnamed), Enums, Option

## Install

* Standart
```
naumi = { git="https://github.com/Paradist/naumi" }
```

* Async (Tokio)
```
naumi = { git="https://github.com/Paradist/naumi", features=["async"] }
```


## Examples


 * Structure to bytes and back
```rust
use naumi::Coder;
use naumi::types::Convert;
use naumi::macros::NaumiConvert;

#[derive(NaumiConvert)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

fn main() -> io::Result<()> {
    let pixel = Pixel {
        r: 12,
        g: 12,
        b: 12,
    };
    
    let mut coder = Coder::new();
    coder.push(&pixel);

    assert_eq!(coder.get_ref(), &vec![12,12,12]);

    assert_eq!(Pixel::from_bytes(&mut coder.get_mut_ref())?, pixel);
    Ok(())
}
```


 * Sending/receiving using TcpStream
```rust
#[derive(NaumiConvert)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("0.0.0.0:80")?;
    
    let mut coder = Coder::new();
    coder.receive(&mut stream)?;
    
    let received_pixel = Pixel::from_bytes(&mut coder.get_mut_ref())?;
    
    // send back
    coder.push(&received_pixel);
    coder.send(&mut stream)?;
    
    Ok(())
}
```
## 🤝 Contributing

Contributions, issues and feature requests are welcome!
Feel free to check [issues page](https://github.com/Paradist/naumi/issues).

## ❤️ Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2023 [Paradist](https://github.com/Paradist).
This project is [MIT](LICENSE) licensed.
