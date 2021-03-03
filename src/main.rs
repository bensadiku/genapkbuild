extern crate clap;
use clap::{App, Arg};

use std::env;
mod zip;

use regex::Regex;

const DEBUG: bool = true;
const SO_CAPTURE_SIZE: usize = 3;
fn main() {
    let matches = App::new("Generate Android make files automatically for APK's")
        .version("1.0")
        .author("Behxhet S. <bensadiku65@gmail.com>")
        .about(
            "Auto generates Android.mk with backwards compatibility before Android 5.0 and after ",
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .required(true)
                .help("Input APK file path you want to generate the mk")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .required(false)
                .help("Name of the APK file you want to generate the mk")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("default")
                .short("d")
                .long("default")
                .required(false)
                .help("Specify the default architecture")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("os")
                .short("o")
                .long("os")
                .required(false)
                .help("Android OS version to generate the mk (semver)")
                .takes_value(true),
        )
        .get_matches();

    // Should never be empty!
    let input = matches.value_of("input").unwrap();
    // If empty, default to input
    let name = matches.value_of("name").unwrap_or(&input);
    // Default archiceture
    let default = matches.value_of("default").unwrap_or("arm x86 arm64");
    // Default to 6.0+
    let os = matches.value_of("os").unwrap_or("6.0");

    let msg = format!("{:?},{:?},{:?},{:?}", input, name, default, os);
    log(msg);

    let apk_dir = env::current_dir().unwrap();
    log(format!("Current dir: {:?}", apk_dir));

    let android_mk_path = apk_dir.join("Android.mk");
    log(format!("Current Android.mk path: {:?}", android_mk_path));

    let files = zip::run(name);

    let re = Regex::new(r"^lib/(.*)/(.*\.so)$").unwrap();

    for file in files {
        let captures = re.captures_iter(&file);

        for cap in captures {
            let capture_size = cap.len();
            log(format!("Cap size: {:?}", capture_size));
            if capture_size == SO_CAPTURE_SIZE {
                log(format!(
                    "Full: {}, Arch: {}, Name: {}",
                    &cap[0], &cap[1], &cap[2]
                ));
            } else {
                log("Unknown capture size")
            }
        }
    }
}

fn log<S>(msg: S)
where
    S: Into<String>,
{
    if DEBUG {
        println!("{}", msg.into());
    }
}
