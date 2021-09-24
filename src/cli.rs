extern crate clap;
use crate::build::BuildSystemBase;
use clap::{App, Arg};

/// Entry point to generating Android.mk
/// Creates a Androidmk struct from input which then allows you..
/// to read architectures supported, .so libraries and more
pub fn read_input() -> BuildSystemBase {
    let matches = App::new("Generate Android.mk / Android.bp automatically from prebuilt APK's")
        .version("1.2.0")
        .author("Behxhet S. <bensadiku65@gmail.com>")
        .about("Generate Android.mk or Android.bp from a prebuilt APK ")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .required(true)
                .help("Input APK file path you want to generate the mk or bp")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .required(false)
                .help("Name of the APK file you want to generate the mk or bp")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("architecture")
                .short("a")
                .long("arch")
                .required(false)
                .help("Specify the default architecture")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("os")
                .short("o")
                .long("os")
                .required(false)
                .help("Android OS version to generate the mk/bp (semver)"),
        )
        .arg(
            Arg::with_name("dexpreopt")
                .short("d")
                .long("dex")
                .required(false)
                .takes_value(true)
                .possible_values(&["true", "false"])
                .hide_possible_values(false)
                .help("To enable or disable pre-optimization. "),
        )
        .arg(
            Arg::with_name("privileged")
                .short("p")
                .long("privileged")
                .required(false)
                .help("Make app privileged (priv-app)"),
        )
        .arg(
            Arg::with_name("extract")
                .short("e")
                .long("extract")
                .required(false)
                .help("Extract .so libs /lib/<abi>/lib<name>.so"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .required(false)
                .help("Enable verbose debug logging"),
        )
        .arg(
            Arg::with_name("soong")
                .short("s")
                .long("soong")
                .required(false)
                .help("Generate Soong Android.bp files from Android apk"),
        )
        .arg(
            Arg::with_name("bazel")
                .short("b")
                .long("bazel")
                .required(false)
                .help("Generate Bazel BUILD files from Android apk"),
        )
        .get_matches();

    // Input path of the apk, should never be empty!
    let input = matches.value_of("input").unwrap();
    // If empty, default to input
    // Get file name without path or ext
    let name = matches.value_of("name").unwrap_or("");
    // Default architecture
    // If not supplied, it will add all architectures found in APK
    let default_architectures = matches.value_of("architecture").unwrap_or("");
    // Did user specify a default architecture
    let has_default_architecture = matches.is_present("architecture");
    // Default to 6.0+
    // Un-used for now!
    let os = matches.value_of("os").unwrap_or("6.0");
    // Privileged app
    let privileged = matches.is_present("privileged");
    // Ability to extract .so libs onto the directory
    let extract_so = matches.is_present("extract");
    // Enable logging
    let debug = matches.is_present("verbose");
    // Generate blueprint instead of makefile
    let bp = matches.is_present("soong");
    // Generate bazel instead of makefile
    let bz = matches.is_present("bazel");
    // Default selection is makefile
    let mk = if !bp && !bz { true } else { false };

    // --dex supports the values ‘true’ or ‘false’ to enable or disable pre-optimization, respectively.
    let has_dex_flag = matches.is_present("dexpreopt");
    let dex_flag = matches.value_of("dexpreopt").unwrap_or("true");
    let dex = (
        // Was the flag passed at all?
        has_dex_flag,
        // If it was, what's the value?
        dex_flag == "true",
    );

    BuildSystemBase::new(
        input,
        name,
        default_architectures,
        has_default_architecture,
        os,
        dex,
        privileged,
        extract_so,
        debug,
        bp,
        mk,
        bz,
    )
}
