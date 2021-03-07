/// Helper methods for tests

#[allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;

use genandroidmk_rs::makefile::Androidmk;

pub fn mk_contains(data: &str) -> bool {
    let mut file = File::open("Android.mk").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents.contains(data);
}

pub fn get_random_mk() -> Androidmk {
    let mk = Androidmk::new(
        "tests/data/multipleArch.apk", // input
        "multipleArch",                // name
        "arm64-v8a",                   // default_architecture
        true,                          // has default architecture
        "6.0",                         // (un-used) os version
        false,                         // pre-optimize dex files
        false,                         // priviledged
    );
    mk
}
