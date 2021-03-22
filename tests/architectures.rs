use genandroidmk_rs::makefile::Androidmk;

mod helper;

use helper::{cleanup_path, file_exists, get_by_name, get_random_mk, mk_contains};

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
fn invalid_abi_panic() {
    // The multipleArch apk supports mutltiple architectures
    // If we set an architecture it doesn't exist, it should panic and exit
    // In this case, armeabi-v5 isn't valid
    let _ = Androidmk::new(
        "tests/data/multipleArch.apk", // input
        "multipleArch",                // name
        "armeabi-v5",                  // default_architecture
        true,                          // has default architecture
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

#[test]
fn multiple_arch_default_extract() {
    cleanup_path("lib/");
    let mut mk = get_by_name("multipleArch");
    mk.set_extract_so(true);
    mk.gen_android_mk();
    let libhello_jnicallback_arm64_v8a = "lib/arm64-v8a/libhello-jnicallback.so";
    let libhello_jnicallback_armeabi_v7a = "lib/armeabi-v7a/libhello-jnicallback.so";
    let libhello_jnicallback_x86 = "lib/x86/libhello-jnicallback.so";
    let libhello_jnicallback_x86_64 = "lib/x86_64/libhello-jnicallback.so";

    assert_eq!(file_exists(libhello_jnicallback_arm64_v8a), true);
    assert_eq!(file_exists(libhello_jnicallback_armeabi_v7a), true);
    assert_eq!(file_exists(libhello_jnicallback_x86), true);
    assert_eq!(file_exists(libhello_jnicallback_x86_64), true);
    cleanup_path("lib/");
}

#[test]
fn multiple_arch_only_armeabi_v7a_extract() {
    cleanup_path("lib/");
    let mut mk = get_by_name("multipleArch");
    mk.set_extract_so(true);
    mk.set_default_architecture("armeabi-v7a".into());
    mk.set_has_default_architecture(true);
    mk.gen_android_mk();
    let libhello_jnicallback_arm64_v8a = "lib/arm64-v8a/libhello-jnicallback.so";
    let libhello_jnicallback_armeabi_v7a = "lib/armeabi-v7a/libhello-jnicallback.so";
    let libhello_jnicallback_x86 = "lib/x86/libhello-jnicallback.so";
    let libhello_jnicallback_x86_64 = "lib/x86_64/libhello-jnicallback.so";

    assert_eq!(file_exists(libhello_jnicallback_arm64_v8a), false);
    assert_eq!(file_exists(libhello_jnicallback_armeabi_v7a), true);
    assert_eq!(file_exists(libhello_jnicallback_x86), false);
    assert_eq!(file_exists(libhello_jnicallback_x86_64), false);
    cleanup_path("lib/");
}

#[test]
fn x86_arch_multiple_so() {
    cleanup_path("lib/");
    let mut mk = get_by_name("x86_multiple_so");
    mk.set_extract_so(true);
    mk.gen_android_mk();
    let libhello_jnicallback_x86 = "lib/x86/libhello-jnicallback.so";
    let liblibtest_gen_x86 = "lib/x86/libtest_gen.so";
    let libhello_jnicallback_arm64_v8a = "lib/arm64-v8a/libhello-jnicallback.so";
    let libhello_jnicallback_x86_64 = "lib/x86_64/libhello-jnicallback.so";

    assert_eq!(file_exists(liblibtest_gen_x86), true);
    assert_eq!(file_exists(libhello_jnicallback_x86), true);
    assert_eq!(file_exists(libhello_jnicallback_arm64_v8a), false);
    assert_eq!(file_exists(libhello_jnicallback_x86_64), false);
    cleanup_path("lib/");
}
