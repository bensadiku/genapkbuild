extern crate clap;
use super::file;
use crate::build::BuildSystemBase;
use clap::{App, Arg};

/// Entry point to generating Android.mk
/// Creates a Androidmk struct from input which then allows you..
/// to read architectures supported, .so libraries and more
pub fn read_input() -> BuildSystemBase {
    let matches = App::new("Generate Android.mk / Android.bp automatically from prebuilt APK's")
        .version("1.2.0")
        .author("Behxhet S. <bensadiku65@gmail.com>")
        .about(
            "Auto generates Android.mk with backwards compatibility before Android 5.0 and after ",
        )
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
                .help(
                    "To enable or disable pre-optimization. Supports the values ‘true’ or ‘false’",
                ),
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
    let name_buf = file::file_name(input);
    let name = matches.value_of("name").unwrap_or(&name_buf);
    // Default architecture
    // If not supplied, it will add all architectures found in APK
    let default_architectures = matches.value_of("architecture").unwrap_or("");
    // Did user specify a default architecture
    let has_default_architecture = matches.is_present("architecture");
    // Pre-optimization
    let dex_opt = matches.is_present("dexpreopt");
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

    BuildSystemBase::new(
        input,
        name,
        default_architectures,
        has_default_architecture,
        os,
        dex_opt,
        privileged,
        extract_so,
        debug,
        bp,
        mk,
        bz,
    )
}
