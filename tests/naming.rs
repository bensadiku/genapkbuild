mod helper;

use helper::{get_random_mk, mk_contains};
use genandroidmk_rs::build::BuildSystemBaseBuilder;

#[test]
fn default_name() {
    let mk = get_random_mk();
    let ret = mk.generate();
    assert_eq!(mk.get_name(), "multipleArch");
    assert_eq!(mk_contains("LOCAL_MODULE := multipleArch"), true);
    assert_eq!(ret, 0);
}

#[test]
fn src_file() {
    let mk = get_random_mk();
    let ret = mk.generate();
    assert_eq!(mk_contains("LOCAL_SRC_FILES := multipleArch.apk"), true);
    assert_eq!(ret, 0);
}

#[test]
fn only_input_empty_name() {
    let mut base = BuildSystemBaseBuilder::new();
    base.set_name("");
    let mk = base.build();
    
    let ret = mk.generate();
    assert_eq!(mk_contains("LOCAL_SRC_FILES := multipleArch.apk"), true);
    assert_eq!(mk_contains("LOCAL_MODULE := multipleArch"), true);
    assert_eq!(ret, 0);
}
