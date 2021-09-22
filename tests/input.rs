use std::env;
mod helper;

use helper::get_random_mk;
#[cfg(test)]
mod tests {
    use super::*;
    use genandroidmk_rs::utils::input_to_abi_vec;

    // run with `cargo test -- --nocapture` for  the logs
    // run with `cargo test -- --test-threads=1` for single threaded tests
    #[test]
    fn multiple_arch_apk() {
        let current_dir = env::current_dir().unwrap();
        println!("current_dir {:?}", current_dir);

        let mk = get_random_mk();
        let ret = mk.generate();
        assert_eq!(mk.get_default_architectures(), vec!["arm64-v8a"]);
        assert_eq!(mk.privileged(), false);
        assert_eq!(mk.get_preopt_dex(), false);
        assert_eq!(helper::mk_contains("LOCAL_DEX_PREOPT"), false);
        assert_eq!(ret, 0);
    }
    #[test]
    fn priviledged_tests() {
        let mut mk = get_random_mk();
        mk.set_privileged(true);
        let ret = mk.generate();
        assert_eq!(helper::mk_contains("LOCAL_PRIVILEGED_MODULE"), true);
        assert_eq!(ret, 0);

        mk.set_privileged(false);
        let ret = mk.generate();
        assert_eq!(helper::mk_contains("LOCAL_PRIVILEGED_MODULE"), false);
        assert_eq!(ret, 0);
    }

    #[test]
    fn pre_opt_dex_tests() {
        let mut mk = get_random_mk();
        mk.set_preopt_dex(true);
        let ret = mk.generate();
        assert_eq!(helper::mk_contains("LOCAL_DEX_PREOPT"), true);
        assert_eq!(ret, 0);

        mk.set_preopt_dex(false);
        let _ret = mk.generate();
        assert_eq!(helper::mk_contains("LOCAL_DEX_PREOPT"), false);
        assert_eq!(ret, 0);
    }

    #[test]
    fn native_so_tests() {
        let mk = get_random_mk();
        let ret = mk.generate();
        let so_files = vec!["libhello-jnicallback.so"];
        assert_eq!(mk.get_libraries(), so_files);
        assert_eq!(ret, 0);
    }

    #[test]
    fn valid_architecture_input() {
        let multiple_arch_input = "armeabi-v7a,x86";
        let arch = vec!["armeabi-v7a", "x86"];
        let parsed = input_to_abi_vec(multiple_arch_input);
        assert_eq!(arch, parsed);
    }

    #[should_panic]
    #[test]
    fn invalid_architecture_input() {
        let multiple_arch_input = "armeabi-v7a,x85";
        let arch = vec!["armeabi-v7a", "x86"];
        let parsed = input_to_abi_vec(multiple_arch_input);
        assert_eq!(arch, parsed);
    }
}
