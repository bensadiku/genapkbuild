mod helper;

use helper::{get_random_mk, mk_contains};
use genandroidmk_rs::makefile::Androidmk;

#[test]
fn default_name() {
    let mk = get_random_mk();
    let _ret = mk.gen_android_mk();
    assert_eq!(mk.get_name(), "multipleArch");
    assert_eq!(mk_contains("LOCAL_MODULE := multipleArch"), true);
}

#[test]
fn src_file() {
    let mk = get_random_mk();
    let _ret = mk.gen_android_mk();
    assert_eq!(mk_contains("LOCAL_SRC_FILES := multipleArch.apk"), true);
}

#[test]
fn only_input_empty_name() {
    let mk = Androidmk::new(
        "tests/data/multipleArch.apk", // input
        "",                            // name
        "arm64-v8a",                   // default_architecture
        true,                          // has default architecture
        "6.0",                         // (un-used) os version
        false,                         // pre-optimize dex files
        false,                         // priviledged
    );
    let _ret = mk.gen_android_mk();
    assert_eq!(mk_contains("LOCAL_SRC_FILES := multipleArch.apk"), true);
    assert_eq!(mk_contains("LOCAL_MODULE := multipleArch"), true);
}
