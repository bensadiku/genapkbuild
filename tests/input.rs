mod helper;

use helper::{get_random_bp, get_random_mk};
#[cfg(test)]
mod tests {
    use super::*;
    use genapkbuild::utils::input_to_abi_vec;

    #[test]
    fn multiple_arch_apk() {
        let mk = get_random_mk();
        let ret = mk.generate();
        assert_eq!(mk.get_default_architectures(), vec!["arm64-v8a"]);
        assert_eq!(mk.privileged(), false);
        assert_eq!(mk.get_preopt_dex().0, false);
        assert_eq!(mk.get_preopt_dex().0, false);
        assert_eq!(helper::mk_contains("LOCAL_DEX_PREOPT"), false);
        assert_eq!(ret, 0);
    }
    #[test]
    fn multiple_arch_apk_bp() {
        let bp = get_random_bp();
        let ret = bp.generate();
        assert_eq!(bp.get_default_architectures(), vec!["arm64-v8a"]);
        assert_eq!(bp.privileged(), false);
        assert_eq!(bp.get_preopt_dex().0, false);
        assert_eq!(bp.get_preopt_dex().0, false);
        assert_eq!(helper::bp_contains("dex_preopt"), false);
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
    fn priviledged_tests_bp() {
        let mut bp = get_random_bp();
        bp.set_privileged(true);
        let ret = bp.generate();
        assert_eq!(helper::bp_contains("privileged"), true);
        assert_eq!(helper::bp_contains("privileged: true,"), true);
        assert_eq!(ret, 0);

        bp.set_privileged(false);
        let ret = bp.generate();
        assert_eq!(helper::bp_contains("privileged"), false);
        assert_eq!(ret, 0);
    }

    #[test]
    fn pre_opt_dex_tests() {
        let mut mk = get_random_mk();
        mk.set_preopt_dex((true, true));
        let ret = mk.generate();
        assert_eq!(helper::mk_contains("LOCAL_DEX_PREOPT := true"), true);
        assert_eq!(ret, 0);

        mk.set_preopt_dex((true, false));
        let ret = mk.generate();
        assert_eq!(helper::mk_contains("LOCAL_DEX_PREOPT := false"), true);
        assert_eq!(ret, 0);

        mk.set_preopt_dex((false, false));
        let ret = mk.generate();
        assert_eq!(helper::mk_contains("LOCAL_DEX_PREOPT"), false);
        assert_eq!(ret, 0);
    }

    #[test]
    fn pre_opt_dex_tests_bp() {
        let mut bp = get_random_bp();
        bp.set_preopt_dex((true, true));
        let ret = bp.generate();
        // Verify we added dex_preopt and enabled it
        assert_eq!(
            helper::bp_contains(
                r#"
    dex_preopt: {
        enabled: true,
    },"#
            ),
            true
        );
        assert_eq!(ret, 0);

        // Verify we added dex_preopt but disabled it
        bp.set_preopt_dex((true, false));
        let ret = bp.generate();
        assert_eq!(
            helper::bp_contains(
                r#"
    dex_preopt: {
        enabled: false,
    },"#
            ),
            true
        );
        assert_eq!(ret, 0);

        // Verify dex_preopt doesn't even exist in the output
        bp.set_preopt_dex((false, false));
        let ret = bp.generate();
        assert_eq!(
            helper::bp_contains(
                r#"
    dex_preopt: {
        enabled: true,
    },"#
            ),
            false
        );
        assert_eq!(
            helper::bp_contains(
                r#"
    dex_preopt: {
        enabled: false,
    },"#
            ),
            false
        );
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
        let bp = false;
        let parsed = input_to_abi_vec(multiple_arch_input, bp);
        assert_eq!(arch, parsed);
    }

    #[test]
    fn valid_architecture_input_bp() {
        let multiple_arch_input = "armeabi-v7a,arm64-v8a,x86";
        let arch = vec!["arm", "arm64", "x86"];
        let bp = true;
        let parsed = input_to_abi_vec(multiple_arch_input, bp);
        assert_eq!(arch, parsed);
    }

    #[test]
    fn valid_architecture_input_mk() {
        let multiple_arch_input = "arm,arm64,x86";
        let arch = vec!["armeabi-v7a", "arm64-v8a", "x86"];
        let bp = false;
        let parsed = input_to_abi_vec(multiple_arch_input, bp);
        assert_eq!(arch, parsed);
    }

    #[should_panic]
    #[test]
    fn invalid_architecture_input() {
        let multiple_arch_input = "armeabi-v7a,x85";
        let arch = vec!["armeabi-v7a", "x86"];
        let parsed = input_to_abi_vec(multiple_arch_input, true);
        assert_eq!(arch, parsed);
    }
}
