#![feature(test)]
extern crate test;

use test::Bencher;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
struct SomeData {
    d1: u8,
    d2: u16,
    d3: u32,
    d4: u64,
}

impl SomeData {
    fn new_rand() -> Self {
        Self {
            d1: fastrand::u8(0..=u8::MAX),
            d2: fastrand::u16(0..=u16::MAX),
            d3: fastrand::u32(0..=u32::MAX),
            d4: fastrand::u64(0..=u64::MAX),
        }
    }

    fn get_x(x: usize) -> Vec<Self> {
        (0..x).map(|_| Self::new_rand()).collect()
    }
}


fn prepare_for_encode() -> Vec<Vec<SomeData>> {
    let rows = 1_000;
    let columns = 1_000;
    let mut res = vec![];

    for _ in 0..rows {
        res.push(SomeData::get_x(columns))
    }
    res
}

fn prepare_for_decode() -> Vec<u8> {
    pot::to_vec(&DATA.clone()).unwrap()
}

lazy_static!(
    static ref DATA: Vec<Vec<SomeData>> = prepare_for_encode();
    static ref DATA_ENCODED: Vec<u8> = prepare_for_decode();
);

#[bench]
fn encode(b: &mut Bencher) {
    b.iter(|| {
        let _value = pot::to_vec(&DATA.clone()).unwrap();
    });;
}

#[bench]
fn decode(b: &mut Bencher) {
    b.iter(|| {
        let data = DATA_ENCODED.clone();
        let _decoded: Vec<Vec<SomeData>> = pot::from_slice(&data).unwrap();
    });;
}

pub fn get_encoded_led() -> usize {
    DATA_ENCODED.len()
}

