#[macro_use]
extern crate criterion;
extern crate cdchunking;
extern crate fast_cdc;

use cdchunking::{Chunker, ZPAQ};
use criterion::Criterion;
use fast_cdc::FastCDC;
use std::fs::File;
use std::io::prelude::*;

fn cdc_benchmark(c: &mut Criterion) {
    // 1. Read File
    let mut file = File::open("./sandbox/bin/cp").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let data2 = data.clone();

    // 2. Create benchmark group
    let mut group = c.benchmark_group("Content Defined Chunking algorithm");

    // 3. Bench fast cdc algorithm
    group.bench_function("FastCDC", |b| {
        b.iter(|| {
            let chunker = Chunker::new(FastCDC {});
            let mut result: Vec<&[u8]> = Vec::new();
            for slice in chunker.slices(&data) {
                result.push(slice);
            }
            // println!("Number of chunk {:?}", result.len());
        })
    });

    // 3. Bench ZPAQ algorithm
    group.bench_function("ZPAQ", |b| {
        b.iter(|| {
            let chunker = Chunker::new(ZPAQ::new(13)); // 13 bits = 8 KiB block average
            let mut result: Vec<&[u8]> = Vec::new();
            for slice in chunker.slices(&data2) {
                result.push(slice);
            }
            // println!("Number of chunk {:?}", result.len());
        })
    });

    group.finish();
}

criterion_group!(benches, cdc_benchmark);
criterion_main!(benches);
