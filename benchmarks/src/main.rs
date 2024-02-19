#![feature(test)]
#![feature(allocator_api)]
extern crate test;

mod naumi_bench;
mod alkahest_bench;
mod cbor_bench;
mod pot_bench;
mod simd_json_bench;

fn main() {
    println!("Naumi encoded len:       {}", naumi_bench::get_encoded_led());
    println!("Alkahest encoded len:    {}", alkahest_bench::get_encoded_led());
    println!("Pot encoded len:         {}", pot_bench::get_encoded_led());
    println!("Cbor encoded len:        {}", cbor_bench::get_encoded_led());
    println!("Simd Json encoded len:   {}", simd_json_bench::get_encoded_led());
}

