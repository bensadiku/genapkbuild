use super::*;
use makefile::Androidmk;

#[test]
fn smol_test() {
    let mk = get_random_mk();
    assert_eq!(mk.get_default_architecture(), "arm64-v8a");


    let _ret2 = mk.gen_android_mk();

}

fn get_random_mk() -> Androidmk {
    let m = Androidmk::new(
        "test/test.apk",
        "test",
        "arm64-v8a",
        true,
        "6.0",
        false,
        false,
    );
    m
}
