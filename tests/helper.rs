use genandroidmk_rs::build::{BuildSystemBase, BuildSystemBaseBuilder};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// Helper methods for tests

// run with `cargo test -- --nocapture` for  the logs
// run with `cargo test -- --test-threads=1` for single threaded tests
pub fn mk_contains(data: &str) -> bool {
    let mut file = File::open("Android.mk").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents.contains(data);
}

pub fn get_random_mk() -> BuildSystemBase {
    let mut mk = BuildSystemBaseBuilder::new();
    mk.build()
}

pub fn get_by_name(name: &str) -> BuildSystemBase {
    let mut mk = BuildSystemBaseBuilder::new();
    mk.set_input(format!("tests/data/{}.apk", name));
    mk.set_name(name);
    mk.build()
}

pub fn file_exists(path: &str) -> bool {
    let so_path = Path::new(path);
    if !so_path.exists() {
        println!("Path {:?} does not exist", so_path);
    }
    return so_path.exists();
}

pub fn cleanup_path(path: &str) {
    std::fs::remove_dir_all(path);
}
