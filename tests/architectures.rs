use genapkbuild::build::BuildSystemBaseBuilder;

mod helper;

use helper::{cleanup_path, file_exists, get_by_name, get_random_mk, mk_contains};

// https://developer.android.com/studio/projects/gradle-external-native-builds#specify-abi

#[test]
fn architectures_tests() {
    let mk = get_random_mk();
    let ret = mk.generate();
    let archs = vec!["arm64-v8a", "armeabi-v7a", "x86", "x86_64"];
    assert_eq!(mk.get_architectures(), archs);
    assert_eq!(ret, 0);
}

#[should_panic]
#[test]
fn invalid_abi_panic() {
    // The multipleArch apk supports mutltiple architectures
    // If we set an architecture it doesn't exist, it should panic and exit
    // In this case, armeabi-v5 isn't valid
    let mut mk = BuildSystemBaseBuilder::new();
    mk.override_arch("armeabi-v5".into());
    mk.build();
}

#[test]
fn default_arch() {
    let mut base = BuildSystemBaseBuilder::new();
    base.override_arch("arm64-v8a".into());
    let mk = base.build();
    let ret = mk.generate();
    assert_eq!(ret, 0);
    assert_eq!(mk.get_default_architectures(), vec!["arm64-v8a"]);
    assert_eq!(mk.has_default_architecture(), true);
    assert_eq!(mk_contains("@lib/arm64-v8a/libhello-jnicallback.so"), true);
}

#[test]
fn force_x86() {
    let mut mk = BuildSystemBaseBuilder::new();
    mk.set_input(format!("tests/data/armeabi-v7a.apk"));
    mk.set_name("armeabi-v7a");
    mk.set_default_architectures(vec!["x86".into()]);
    mk.set_has_default_architecture(true);
    let base = mk.build();
    let ret = base.generate();
    assert_eq!(base.get_default_architectures(), vec!["x86"]);
    assert_eq!(mk_contains("@lib/x86/libhello-jnicallback.so"), true);
    assert_eq!(ret, 0);
}

#[test]
fn default_x86() {
    let mk = get_by_name("x86");
    let ret = mk.generate();
    assert_eq!(mk.get_default_architectures(), vec!["x86"]);
    assert_eq!(mk_contains("@lib/x86/libhello-jnicallback.so"), true);
    assert_eq!(ret, 0);
}

#[test]
fn default_x86_2() {
    let mk = get_by_name("x86_multiple_so");
    let ret = mk.generate();
    assert_eq!(ret, 0);
    assert_eq!(mk.get_default_architectures(), vec!["x86"]);
    assert_eq!(mk_contains("@lib/x86/libhello-jnicallback.so"), true);
}

#[test]
fn default_armeabi_v7a() {
    let mk = get_by_name("armeabi-v7a");
    println!("benitest {:?}", mk.get_default_architectures());
    let ret = mk.generate();
    assert_eq!(ret, 0);
    assert_eq!(mk.get_default_architectures(), vec!["armeabi-v7a"]);
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
    let ret = mk.generate();
    let libhello_jnicallback_arm64_v8a = "lib/arm64-v8a/libhello-jnicallback.so";
    let libhello_jnicallback_armeabi_v7a = "lib/armeabi-v7a/libhello-jnicallback.so";
    let libhello_jnicallback_x86 = "lib/x86/libhello-jnicallback.so";
    let libhello_jnicallback_x86_64 = "lib/x86_64/libhello-jnicallback.so";

    assert_eq!(file_exists(libhello_jnicallback_arm64_v8a), true);
    assert_eq!(file_exists(libhello_jnicallback_armeabi_v7a), true);
    assert_eq!(file_exists(libhello_jnicallback_x86), true);
    assert_eq!(file_exists(libhello_jnicallback_x86_64), true);
    cleanup_path("lib/");
    assert_eq!(ret, 0);
}

#[test]
fn multiple_arch_only_armeabi_v7a_extract() {
    cleanup_path("lib/");
    let mut mk = BuildSystemBaseBuilder::new();
    mk.set_input(format!("tests/data/multipleArch.apk"));
    mk.set_name("multipleArch");
    mk.set_default_architectures(vec!["armeabi-v7a".into()]);
    mk.set_extract_so(true);
    mk.set_has_default_architecture(true);

    let ret = mk.build().generate();
    let libhello_jnicallback_arm64_v8a = "lib/arm64-v8a/libhello-jnicallback.so";
    let libhello_jnicallback_armeabi_v7a = "lib/armeabi-v7a/libhello-jnicallback.so";
    let libhello_jnicallback_x86 = "lib/x86/libhello-jnicallback.so";
    let libhello_jnicallback_x86_64 = "lib/x86_64/libhello-jnicallback.so";

    assert_eq!(file_exists(libhello_jnicallback_arm64_v8a), false);
    assert_eq!(file_exists(libhello_jnicallback_armeabi_v7a), true);
    assert_eq!(file_exists(libhello_jnicallback_x86), false);
    assert_eq!(file_exists(libhello_jnicallback_x86_64), false);
    cleanup_path("lib/");
    assert_eq!(ret, 0);
}

#[test]
fn x86_arch_multiple_so() {
    cleanup_path("lib/");
    let mut mk = get_by_name("x86_multiple_so");
    mk.set_extract_so(true);
    let ret = mk.generate();
    let libhello_jnicallback_x86 = "lib/x86/libhello-jnicallback.so";
    let liblibtest_gen_x86 = "lib/x86/libtest_gen.so";
    let libhello_jnicallback_arm64_v8a = "lib/arm64-v8a/libhello-jnicallback.so";
    let libhello_jnicallback_x86_64 = "lib/x86_64/libhello-jnicallback.so";

    assert_eq!(file_exists(liblibtest_gen_x86), true);
    assert_eq!(file_exists(libhello_jnicallback_x86), true);
    assert_eq!(file_exists(libhello_jnicallback_arm64_v8a), false);
    assert_eq!(file_exists(libhello_jnicallback_x86_64), false);
    cleanup_path("lib/");
    assert_eq!(ret, 0);
}
