use fs::File;
use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::file;
use super::makefile::Androidmk;
use super::zip::extract_zip;

/// Return file name with extension from a path
pub fn file_name_ext(path: &str) -> String {
    let path = Path::new(path);

    let name = path.file_name().unwrap_or(OsStr::new(""));

    let filename = name.to_os_string();

    if let Some(name_string) = filename.to_str() {
        name_string.into()
    } else {
        panic!(format!("Could not get file name for {:?}", filename));
    }
}

/// Return only file name from a path
pub fn file_name(path: &str) -> String {
    let path = Path::new(path);

    let name = path.file_stem().unwrap_or(OsStr::new(""));

    let filename = name.to_os_string();

    if let Some(name_string) = filename.to_str() {
        name_string.into()
    } else {
        panic!(format!("Could not get file name for {:?}", filename));
    }
}

pub fn gen_android_mk_con(mk: &Androidmk) {
    let mk_path = mk.get_mk_path();
    let display = mk_path.display();
    let apk_dir: PathBuf = file::file_name_ext(&mk.get_input()).into();
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&mk.get_mk_path()) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let mut mk_file_content: String = String::new();
    mk_file_content.push_str("LOCAL_PATH := $(call my-dir)\n");
    mk_file_content.push_str("\nmy_archs := arm x86 arm64\n");
    mk_file_content.push_str("my_src_arch := $(call get-prebuilt-src-arch, $(my_archs))\n");
    mk_file_content.push_str("\ninclude $(CLEAR_VARS)\n");
    mk_file_content.push_str(&format!("LOCAL_MODULE := {}\n", mk.get_name()));
    mk_file_content.push_str("LOCAL_MODULE_CLASS := APPS\n");
    mk_file_content.push_str("LOCAL_MODULE_TAGS := optional\n");
    mk_file_content.push_str("LOCAL_BUILT_MODULE_STEM := package.apk\n");
    mk_file_content.push_str("LOCAL_MODULE_SUFFIX := $(COMMON_ANDROID_PACKAGE_SUFFIX)\n");
    mk_file_content.push_str("LOCAL_CERTIFICATE := PRESIGNED\n");

    if mk.privileged() {
        mk_file_content.push_str(&format!("LOCAL_PRIVILEGED_MODULE := {}\n", "true"));
    }
    if mk.get_preopt_dex() {
        mk_file_content.push_str(&format!("LOCAL_DEX_PREOPT := {}\n", mk.get_preopt_dex()));
    }

    mk_file_content.push_str(&format!("LOCAL_SRC_FILES := {}\n", apk_dir.display()));

    let native_libraries = mk.get_libraries();
    let architecture = mk.get_default_architecture();
    if native_libraries.len() > 0 {
        mk_file_content.push_str("\n");
        mk_file_content.push_str("LOCAL_PREBUILT_JNI_LIBS := \\\n");

        let lib_type = if mk.extract_so() {
            //TODO: maybe rm -rf * this dir before extracting?
            extract_zip(mk);
            // extracted libs
            " lib"
        } else {
            // not extracted
            " @lib"
        };

        for key in native_libraries {
            mk_file_content.push_str(&format!("  {}/{}/{}", lib_type, architecture, key));
            mk_file_content.push_str(" \\\n");
        }
    } else {
        mk.log("No native libraries found!");
    }

    mk_file_content.push_str("\nLOCAL_MODULE_TARGET_ARCH := $(my_src_arch)\n");
    mk_file_content.push_str("\ninclude $(BUILD_PREBUILT)\n");

    // Write everything
    match file.write_all(mk_file_content.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully created Android.mk to {}", display),
    }
}

pub fn get_ndk_libs(file_names: Vec<String>) -> (Vec<String>, Vec<String>) {
    let re = Regex::new(r"^lib/(.*)/(.*\.so)$").unwrap();
    const SO_CAPTURE_SIZE: usize = 3;
    let mut architectures: Vec<String> = Vec::new();
    let mut so_paths: Vec<String> = Vec::new();

    for file in file_names {
        let captures = re.captures_iter(&file);

        for cap in captures {
            let capture_size = cap.len();
            // log(format!("Cap size: {:?}", capture_size));
            if capture_size == SO_CAPTURE_SIZE {
                let arch = &cap[1];
                let name = &cap[2];

                architectures.push(arch.to_owned());
                so_paths.push(name.to_owned());

                //log(format!("Path: {}, Arch: {}, Name: {}", &path, &arch, &name));
            } else {
                println!("Unknown capture size");
            }
        }
    }
    architectures.sort();
    architectures.dedup();

    so_paths.sort();
    so_paths.dedup();
    (architectures, so_paths)
}
