mod helper;

use helper::{get_by_name, mk_contains};

#[test]
fn native_libs_tests() {
    let mk = get_by_name("x86_multiple_so");
    let archs = vec!["x86"];
    let libs_extracted = mk.get_libraries();
    println!("libs extracted {:?}", libs_extracted);
    let libs_in_apk = vec!["libhello-jnicallback.so", "libtest_gen.so"];
    assert_eq!(mk.get_architectures(), archs);
    assert_eq!(libs_extracted, libs_in_apk);

    mk.gen_android_mk();
    let mk_so = "@lib/x86/libhello-jnicallback.so \\";
    let mk_so_test = "@lib/x86/libhello-jnicallback.so \\";
    assert_eq!(mk_contains(mk_so), true);
    assert_eq!(mk_contains(mk_so_test), true);
}

#[test]
fn native_lib_arm_v7a_tests() {
    let mk = get_by_name("armeabi-v7a");
    let archs = vec!["armeabi-v7a"];
    let libs_extracted = mk.get_libraries();
    println!("libs extracted {:?}", libs_extracted);
    let libs_in_apk = vec!["libhello-jnicallback.so"];
    assert_eq!(mk.get_architectures(), archs);
    assert_eq!(libs_extracted, libs_in_apk);

    mk.gen_android_mk();
    let mk_so = "@lib/armeabi-v7a/libhello-jnicallback.so ";
    assert_eq!(mk_contains(mk_so), true);

}
