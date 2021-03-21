use genandroidmk_rs::makefile::Androidmk;

mod helper;

use helper::{get_by_name, get_random_mk, mk_contains};

// https://developer.android.com/studio/projects/gradle-external-native-builds#specify-abi

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
        false,                         // if should extract .so libs
        true,                          // debug flag
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

#[test]
fn default_x86() {
    let mk = get_by_name("x86");
    let _ret2 = mk.gen_android_mk();
    assert_eq!(mk.get_default_architecture(), "x86");
    assert_eq!(mk_contains("@lib/x86/libhello-jnicallback.so"), true);
}

#[test]
fn default_x86_2() {
    let mk = get_by_name("x86_multiple_so");
    let _ret2 = mk.gen_android_mk();
    assert_eq!(mk.get_default_architecture(), "x86");
    assert_eq!(mk_contains("@lib/x86/libhello-jnicallback.so"), true);
}

#[test]
fn default_armeabi_v7a() {
    let mk = get_by_name("armeabi-v7a");
    let _ret2 = mk.gen_android_mk();
    assert_eq!(mk.get_default_architecture(), "armeabi-v7a");
    assert_eq!(
        mk_contains("@lib/armeabi-v7a/libhello-jnicallback.so"),
        true
    );
}
