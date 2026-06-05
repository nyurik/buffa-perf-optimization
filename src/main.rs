use std::env;
use std::hint::black_box;
use std::time::Instant;

use buffa::{Message as _, MessageView as _};

mod generated {
    include!("generated/mod.rs");
}
use generated::perf::{Feature, Tile, TileView as TileViewGenerated};

mod fixed {
    include!("fixed/mod.rs");
}
use fixed::perf::TileView as TileViewFixed;

const PATH: &str = "packed.bin";
const FEATURE_COUNT: usize = 65_536;
const VALUES_PER_FEATURE: usize = 16;
const ITERS: usize = 1_000;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("create") => create(),
        Some("time-generated") => time(parse_generated),
        Some("time-fixed") => time(parse_fixed),
        _ => panic!("usage: buffa-perf create | time-generated | time-fixed"),
    }
}

fn create() {
    let feature = Feature {
        values: vec![1; VALUES_PER_FEATURE],
        ..Default::default()
    };
    let msg = Tile {
        features: vec![feature; FEATURE_COUNT],
        ..Default::default()
    };
    let data = msg.encode_to_vec();
    std::fs::write(PATH, &data).expect("write protobuf file");
    println!("wrote {} bytes to {PATH}", data.len());
}

fn time<F: Fn(&[u8]) -> usize>(parse: F) {
    let data = std::fs::read(PATH).expect("read protobuf file");
    let started = Instant::now();
    let mut checksum = 0_usize;
    for _ in 0..ITERS {
        checksum = checksum.wrapping_add(parse(black_box(&data)));
    }
    let elapsed = started.elapsed();
    println!(
        "decoded {} bytes {ITERS} times in {:.3}s; checksum={checksum}",
        data.len(),
        elapsed.as_secs_f64()
    );
}

fn parse_generated(data: &[u8]) -> usize {
    let view = TileViewGenerated::decode_view(data).expect("decode protobuf view");
    black_box(view)
        .features
        .iter()
        .map(|feature| feature.values.len())
        .sum()
}

fn parse_fixed(data: &[u8]) -> usize {
    let view = TileViewFixed::decode_view(data).expect("decode protobuf view");
    black_box(view)
        .features
        .iter()
        .map(|feature| feature.values.len())
        .sum()
}
