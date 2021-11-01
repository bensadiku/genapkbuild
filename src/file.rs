use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;
use super::utils::parse_abi_for_buildsystem;

/// Return file name with extension from a path
pub fn file_name_ext(path: &str) -> String {
    let path = Path::new(path);

    let name = path.file_name().unwrap_or(OsStr::new(""));

    let filename = name.to_os_string();

    if let Some(name_string) = filename.to_str() {
        name_string.into()
    } else {
        panic!("Could not get file name for {:?}", filename);
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
        panic!("Could not get file name for {:?}", filename);
    }
}

pub fn get_ndk_libs(file_names: Vec<String>, bp: bool) -> (Vec<String>, Vec<String>) {
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

    let abis = parse_abi_for_buildsystem(architectures, bp);
    (abis, so_paths)
}
