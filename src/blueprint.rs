use super::build::BuildSystem;
use super::build::BuildSystemBase;
use super::file;
use fs::File;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BluePrint {
    /// The base build system struct
    pub build_system: BuildSystemBase,
}

impl BluePrint {
    pub fn get_default_architectures(&self) -> Vec<String> {
        self.build_system.get_default_architectures().clone()
    }

    pub fn get_name(&self) -> String {
        self.build_system.get_name().clone()
    }

    pub fn has_default_architecture(&self) -> bool {
        self.build_system.has_default_architecture()
    }

    pub fn get_architectures(&self) -> Vec<String> {
        self.build_system.get_architectures().clone()
    }
}

impl BuildSystem for BluePrint {
    fn generate(&self) -> i32 {
        let apk_dir = env::current_dir().unwrap();
        let file_name = "Android.bp";
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

        let lib_size = build_system.get_libraries().len();

        // If we passed some architectures via cli, prioritize those
        // Else, use the architectures we found in APK
        let arch = if self.has_default_architecture() {
            self.get_default_architectures()
        } else {
            self.get_architectures()
        };

        // TODO: Clean this up, it's unreadable, use r#
        if lib_size > 0 {
            println!("Please place APK's inside /prebuilt/<arch>/ \nSee [https://github.com/bensadiku/genapkbuild/issues/6] \n");
            jni_libs.push_str("\n\tarch: {");

            for archi in &arch {
                jni_libs.push_str(&format!("\n\t\t{}: {}", archi, "{"));
                jni_libs.push_str("\n\t\t");
                let path = &format!("\t{}/{}/{}.apk\",\n", "prebuilt", archi, self.get_name());

                jni_libs.push_str(path);
                jni_libs.push_str("\t\t},");
            }
            jni_libs.push_str("\n\t},");
        }

        let dex = if build_system.get_preopt_dex().0 {
            format!(
                r#"dex_preopt: {{
        enabled: {},
    }},"#,
                build_system.get_preopt_dex().1
            )
        } else {
            String::new()
        };

        let priv_app = if build_system.privileged() {
            "privileged: true,"
        } else {
            ""
        };

        let bp = format!(
            r#"android_app_import {{
    name: {:#?},
    srcs: [{:#?}],
    certificate: "presigned",
    {}
    {}
    {}
}}
    "#,
            build_system.get_name(),
            file_name_ext.display(),
            priv_app,
            dex,
            jni_libs
        );

        // Write everything
        let ret = match file.write_all(bp.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => {
                println!("Successfully created Android.bp to {}", display);
                0
            }
        };
        ret
    }
}
