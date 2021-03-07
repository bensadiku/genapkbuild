use genandroidmk_rs::makefile::Androidmk;

mod helper;

use helper::{get_random_mk, mk_contains};

#[test]
fn architectures_tests() {
    let mk = get_random_mk();
    let _ret2 = mk.gen_android_mk();
    let archs = vec!["arm64-v8a", "armeabi-v7a", "x86", "x86_64"];
    assert_eq!(mk.get_architectures(), archs);
}

#[should_panic]
#[test]
fn more_than_one_architecture_panic() {
    // The multipleArch apk supports mutltiple architectures
    // and we haven't set a default architecture here
    let _ = Androidmk::new(
        "tests/data/multipleArch.apk", // input
        "multipleArch",                // name
        "",                            // default_architecture
        false,                         // has default architecture
        "6.0",                         // (un-used) os version
        false,                         // pre-optimize dex files
        false,                         // priviledged
    );
}

#[test]
fn default_arch() {
    let mk = get_random_mk();
    let _ret2 = mk.gen_android_mk();
    assert_eq!(mk.get_default_architecture(), "arm64-v8a");
    assert_eq!(mk.has_default_architecture(), true);
    assert_eq!(mk_contains("@lib/arm64-v8a/libhello-jnicallback.so"), true);
}

#[test]
fn force_x86() {
    let mut mk = get_random_mk();
    mk.set_default_architecture("x86".into());
    let _ret2 = mk.gen_android_mk();
    assert_eq!(mk.get_default_architecture(), "x86");
    assert_eq!(mk_contains("@lib/x86/libhello-jnicallback.so"), true);
}
