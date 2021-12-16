extern crate cdchunking;
extern crate fast_cdc;

use cdchunking::Chunker;
use fast_cdc::FastCDC;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn main() -> Result<(), std::io::Error> {
    let chunker = Chunker::new(FastCDC {});

    let mut file = File::open("./sandbox/bin/cp")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    for slice in chunker.slices(&data) {
        println!("chunk len {:?}", slice.len());
    }

    Ok(())
}
