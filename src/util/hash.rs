use std::fs::File;
use std::io;

use sha2::{Digest, Sha256};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::generic_array::typenum::U32;

pub fn fsha256(mut f: File) -> GenericArray<u8, U32> {
    let mut hasher = Sha256::new();
    io::copy(&mut f, &mut hasher).expect("Could not copy file content.");
    hasher.result()
}