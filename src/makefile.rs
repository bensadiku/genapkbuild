use super::build::BuildSystem;
use super::build::BuildSystemBase;
use super::file;
use super::zip::extract_zip_mk;
use fs::File;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MakeFile {
    /// The base build system struct
    pub build_system: BuildSystemBase,
}

impl MakeFile {
    pub fn get_default_architectures(&self) -> Vec<String> {
        self.build_system.get_default_architectures().clone()
    }

    pub fn get_input(&self) -> String {
        self.build_system.get_input().clone()
    }

    pub fn has_default_architecture(&self) -> bool {
        self.build_system.has_default_architecture()
    }

    pub fn get_architectures(&self) -> Vec<String> {
        self.build_system.get_architectures().clone()
    }

    pub fn log<S>(&self, msg: S)
    where
        S: Into<String>,
    {
        self.build_system.log(msg);
    }
}

impl BuildSystem for MakeFile {
    fn generate(&self) -> i32 {
        // TODO: Handle unwrap gracefully
        let apk_dir = env::current_dir().unwrap();
        let file_name = "Android.mk";
        let android_gen_path: PathBuf = apk_dir.join(file_name);
        let display = android_gen_path.display();
        let file_name_ext: PathBuf = file::file_name_ext(&self.build_system.get_input()).into();
        let build_system = self.build_system.clone();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&android_gen_path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        let mut mk_file_content: String = String::new();
        mk_file_content.push_str("LOCAL_PATH := $(call my-dir)\n");
        mk_file_content.push_str("\nmy_archs := arm x86 arm64\n");
        mk_file_content.push_str("my_src_arch := $(call get-prebuilt-src-arch, $(my_archs))\n");
        mk_file_content.push_str("\ninclude $(CLEAR_VARS)\n");
        mk_file_content.push_str(&format!("LOCAL_MODULE := {}\n", build_system.get_name()));
        mk_file_content.push_str("LOCAL_MODULE_CLASS := APPS\n");
        mk_file_content.push_str("LOCAL_MODULE_TAGS := optional\n");
        mk_file_content.push_str("LOCAL_BUILT_MODULE_STEM := package.apk\n");
        mk_file_content.push_str("LOCAL_MODULE_SUFFIX := $(COMMON_ANDROID_PACKAGE_SUFFIX)\n");
        mk_file_content.push_str("LOCAL_CERTIFICATE := PRESIGNED\n");
        if build_system.privileged() {
            mk_file_content.push_str(&format!("LOCAL_PRIVILEGED_MODULE := {}\n", "true"));
        }
        if build_system.get_preopt_dex() {
            mk_file_content.push_str(&format!(
                "LOCAL_DEX_PREOPT := {}\n",
                build_system.get_preopt_dex()
            ));
        }
        mk_file_content.push_str(&format!("LOCAL_SRC_FILES := {}\n", file_name_ext.display()));
        let native_libraries = build_system.get_libraries();
        let lib_size = native_libraries.len();
        //If we have some native libs, start writing to makefile for them
        if lib_size > 0 {
            let lib_type = if self.build_system.extract_so() {
                //TODO: maybe rm -rf * this dir before extracting?
                extract_zip_mk(self);
                // extracted libs
                " lib"
            } else {
                // not extracted
                " @lib"
            };
            mk_file_content.push_str("\n");
            mk_file_content.push_str("LOCAL_PREBUILT_JNI_LIBS := \\\n");
            // If we passed some architectures via cli, prioritize those
            // Else, use the architectures we found in APK
            // TODO: These are redundant, pls clean up
            let arch = if build_system.has_default_architecture() {
                build_system.get_default_architectures()
            } else {
                build_system.get_architectures()
            };
            let arch_size = arch.len();
            for (i, archi) in arch.iter().enumerate() {
                for (j, lib) in native_libraries.iter().enumerate() {
                    mk_file_content.push_str(&format!("  {}/{}/{}", lib_type, archi, lib));
                    // If it's the last iteration, simply add a new line
                    if i + 1 == arch_size && j + 1 == lib_size {
                        mk_file_content.push_str(" \n");
                    } else {
                        mk_file_content.push_str(" \\\n");
                    }
                }
            }
        } else {
            self.log("No native libraries found!");
        }
        mk_file_content.push_str("\nLOCAL_MODULE_TARGET_ARCH := $(my_src_arch)\n");
        mk_file_content.push_str("\ninclude $(BUILD_PREBUILT)\n");
        // Write everything
        match file.write_all(mk_file_content.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => {
                println!("Successfully created Android.mk to {}", display);
                0
            }
        }
    }
}
