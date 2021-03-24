/// Should check if the inputted ABI's are valid
/// Convert the String to a list of ABI's
pub fn input_to_abi_vec(input: &str) -> Vec<String> {
    if input.is_empty() {
        return vec![];
    }
    let list: Vec<String> = input.split(",").map(|s| s.to_string()).collect();
    for abi in &list {
        // If there's a default architecture supplied but it's not a valid one
        // e.g a typo, exit immediately, see issue #1
        if !VALID_ABI.contains(&abi.as_ref()) {
            panic!(
                "{} is not a valid ABI. Use commas to separate them e.g armeabi-v7a,arm64-v8a \n\n Must be one of {:?}",
                abi, VALID_ABI
            );
        }
    }
    return list;
}

pub const VALID_ABI: &[&str] = &["armeabi-v7a", "arm64-v8a", "x86", "x86_64"];
