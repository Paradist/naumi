<h1 align=center>Naumi</h1>
<p align=center>Lightweight and fast data transfer protocol. Fast de/serialization and tiny size!</p>

## Why?

Naumi is an innovative data format designed to be a complete alternative to JSON and ProtoBuf, while providing greater efficiency and flexibility. It is designed to meet modern data processing requirements, offering compact serialization, improved read and write speeds, and support for complex data structures and types, making it an ideal choice for a wide range of applications, from webAPIs to real-time messaging and configuration files.


## Benchmarks
**Benchmark sources: [Benchmarks](https://github.com/Paradist/naumi/tree/main/benchmarks)**

|      | **Encode** | **Decode** | **Size** |
|:------------:|:----------:|:----------:|:--------:|
|   **Naumi**  |   6,4 ms   |   6,8 ms   | 15002002 |
| **Alkahest** |   24,3 ms  |   22,7 ms  | 15008008 |
|    **Pot**   |  72,64 ms  |  187,9 ms  | 23994125 |
|   **Cbor**   |   40,8 ms  |  115,2 ms  | 31904769 |
| **SimdJson** |  107,6 ms  |  466,3 ms  | 62543210 |

## 📝 To Do (before the 1.0.0 release)

- [X] Describe the functions with examples.
- [ ] Data compression using zstd
- [ ] TLS
- [ ] More speed and optimization!
  - [X] Huge speedup of iterators and custom types.
  - [X] Huge speedup of numbers decode.
  - [ ] Let me know on the [issues page](https://github.com/Paradist/naumi/issues).
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
naumi = "0.2"
```

* Async (Tokio)
```
naumi = { version = "0.2", features = ["async"] }
```


## Examples


 * Structure to bytes and back
```rust
use naumi::Coder;
use naumi::types::Convert;
use naumi::nmacro::NaumiConvert;

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
## Update log

 * `0.2.4` - Added benchmarks.
 * `0.2.3` - Accelerating the decode of numbers.
 * `0.2.2` - Bug fixes
 * `0.2.1` - Describe the functions with examples.
 * `0.2.0` - Hugely accelerated iterators and  custom structures, bug fixes.
 * `0.1.0 - 0.1.5` - Bug fixes


## 🤝 Contributing

Contributions, issues and feature requests are welcome!
Feel free to check [issues page](https://github.com/Paradist/naumi/issues).

## ❤️ Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2024 [Paradist](https://github.com/Paradist).
This project is [MIT](LICENSE) licensed.
