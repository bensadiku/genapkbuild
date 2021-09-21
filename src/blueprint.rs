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

        let native_libraries = build_system.get_libraries();
        let lib_size = native_libraries.len();
        //If we have some native libs, panic for now
        if lib_size > 0 {
            // FIXME: implement gen
            panic!("BP generation for jni libs not supported yet see [https://github.com/bensadiku/genandroidmk_rs/issues/6]");
        }

        let bp = format!(
            r#"android_app_import {{
    name: {:#?},
    srcs: [{:#?}],
    certificate: "presigned",
    privileged: {},
    dex_preopt: {{
        enabled: {},
    }},
}}
    "#,
            build_system.get_name(),
            file_name_ext.display(),
            build_system.privileged(),
            build_system.get_preopt_dex(),
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
