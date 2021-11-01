/// Should check if the inputted ABI's are valid
/// Convert the String to a list of ABI's
pub fn input_to_abi_vec(input: &str, bp: bool) -> Vec<String> {
    if input.is_empty() {
        return vec![];
    }
    let list: Vec<String> = input.split(",").map(|s| s.to_string()).collect();

    return parse_abi_for_buildsystem(list, bp);
}

/// We customize ABI based on build system. e.g. makefile uses arm64-v8a, blueprint uses arm64
pub fn parse_abi_for_buildsystem(dirty_abis: Vec<String>, bp: bool) -> Vec<String> {
    let mut filtered_list: Vec<String> = Vec::new();
    for (_i, abi) in dirty_abis.iter().enumerate() {
        // If there's a default architecture supplied but it's not a valid one
        // e.g a typo, exit immediately, see issue #1
        if !VALID_ABI.contains(&abi.as_ref()) {
            panic!(
                "{} is not a valid ABI. Use commas to separate them e.g armeabi-v7a,arm64-v8a \n\n Must be one of {:?}",
                abi, VALID_ABI
            );
        }
        if filtered_list.contains(abi) {
            println!("{} is already passed once, ignoring", abi);
            continue;
        }
        // small mistakes that can be passed through the cli, correct them
        if !bp {
            if abi == "arm64" {
                filtered_list.push("arm64-v8a".to_string());
            } else if abi == "arm" {
                filtered_list.push("armeabi-v7a".to_string());
            } else {
                filtered_list.push(abi.to_string());
            }
        } else {
            if abi == "arm64-v8a" {
                filtered_list.push("arm64".to_string());
            } else if abi == "armeabi-v7a" {
                filtered_list.push("arm".to_string());
            } else {
                filtered_list.push(abi.to_string());
            }
        }
    }
    filtered_list
}

pub const VALID_ABI: &[&str] = &["armeabi-v7a", "arm64-v8a", "x86", "arm", "arm64", "x86_64"];
