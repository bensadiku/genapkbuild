mod helper;

use genapkbuild::build::BuildSystemBaseBuilder;
use helper::{bp_contains, get_random_bp, get_random_mk, mk_contains};

#[test]
fn default_name() {
    let mk = get_random_mk();
    let ret = mk.generate();
    assert_eq!(mk.get_name(), "multipleArch");
    assert_eq!(mk_contains("LOCAL_MODULE := multipleArch"), true);
    assert_eq!(ret, 0);
}

#[test]
fn default_name_bp() {
    let bp = get_random_bp();
    let ret = bp.generate();
    assert_eq!(bp.get_name(), "multipleArch");
    assert_eq!(bp_contains(r#"name: "multipleArch","#), true);
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
fn src_file_bp() {
    let bp = get_random_bp();
    let ret = bp.generate();
    assert_eq!(bp_contains(r#"srcs: ["multipleArch.apk"],"#), true);
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

#[test]
fn only_input_empty_name_bp() {
    let mut base = BuildSystemBaseBuilder::new();
    base.set_name("");
    base.set_blueprint(true);
    base.set_make_file(false);
    let bp = base.build();
    let ret = bp.generate();
    assert_eq!(bp_contains(r#"name: "multipleArch","#), true);
    assert_eq!(bp_contains(r#"srcs: ["multipleArch.apk"],"#), true);
    assert_eq!(ret, 0);
}
