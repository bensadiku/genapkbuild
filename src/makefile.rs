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

        let mut jni_libs: String = String::new();
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
            jni_libs.push_str("\n");
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
                    jni_libs.push_str(&format!("  {}/{}/{}", lib_type, archi, lib));
                    // If it's the last iteration, simply add a new line
                    if i + 1 == arch_size && j + 1 == lib_size {
                        jni_libs.push_str(" \n");
                    } else {
                        jni_libs.push_str(" \\\n");
                    }
                }
            }
        } else {
            self.log("No native libraries found!");
        }

        // If the dex flag was passed
    
        let dex = if build_system.get_preopt_dex().0 {
            format!("LOCAL_DEX_PREOPT := {}", build_system.get_preopt_dex().1)
        } else {
            String::new()
        };

        let priv_app = if build_system.privileged() {
            "LOCAL_PRIVILEGED_MODULE := true"
        } else {
            ""
        };

        let mk_file_content = format!(
            r#"
LOCAL_PATH := $(call my-dir)

my_archs := arm x86 arm64
my_src_arch := $(call get-prebuilt-src-arch, $(my_archs))

include $(CLEAR_VARS)
LOCAL_MODULE := {}
LOCAL_MODULE_CLASS := APPS
LOCAL_MODULE_TAGS := optional
LOCAL_BUILT_MODULE_STEM := package.apk
LOCAL_MODULE_SUFFIX := $(COMMON_ANDROID_PACKAGE_SUFFIX)
LOCAL_CERTIFICATE := PRESIGNED
LOCAL_SRC_FILES := {}
{}
{}

LOCAL_PREBUILT_JNI_LIBS := {}

LOCAL_MODULE_TARGET_ARCH := $(my_src_arch)

include $(BUILD_PREBUILT)
    "#,
            build_system.get_name(),
            file_name_ext.display(),
            dex,
            priv_app,
            jni_libs,
        );
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
