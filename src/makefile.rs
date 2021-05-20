extern crate clap;
use clap::{App, Arg};

use std::env;
use std::path::PathBuf;

use super::file;
use super::utils;
use super::zip;

#[derive(Debug, Clone)]
pub struct Androidmk {
    input: String,
    name: String,
    default_architectures: Vec<String>,
    has_default_architecture: bool,
    os: String,
    preopt_dex: bool,
    privileged: bool,
    extract_so: bool,
    debug: bool,
    blueprint: bool,
    android_mk_path: PathBuf,
    apk_path: PathBuf,
    libraries: Vec<String>,
    architectures: Vec<String>,
}

impl Androidmk {
    pub fn new<I, N, D, O>(
        input: I,
        name: N,
        default: D,
        has_default: bool,
        os: O,
        preopt_dex: bool,
        privileged: bool,
        extract_so: bool,
        debug: bool,
        bp: bool,
    ) -> Androidmk
    where
        I: Into<String>,
        N: Into<String>,
        D: Into<String>,
        O: Into<String>,
    {
        let apk_dir = env::current_dir().unwrap();
        //log(format!("Current dir: {:?}", apk_dir));
        let file_name = if bp { "Android.bp" } else { "Android.mk" };
        let android_gen_path = apk_dir.join(file_name);
        // log(format!("Current Android.mk path: {:?}", &android_mk_path));

        let mut name_string = name.into();
        let input_string = input.into();

        // If name supplied was empty, add it from the path
        // See `only_input_empty_name` test
        if name_string.is_empty() {
            name_string = file::file_name(&input_string);
        }

        let default_architectures = utils::input_to_abi_vec(&default.into());

        let mut m = Self {
            input: input_string,
            name: name_string,
            default_architectures: default_architectures,
            has_default_architecture: has_default,
            os: os.into(),
            preopt_dex: preopt_dex,
            privileged: privileged,
            extract_so: extract_so,
            debug: debug,
            blueprint: bp,
            android_mk_path: android_gen_path,
            apk_path: apk_dir,
            libraries: Vec::new(),
            architectures: Vec::new(),
        };
        m.init();
        m
    }

    /// Handles initialization of some variable
    /// Reads apk files, extracts .so files and adds them as list
    /// Reads architectures, extracts them as list
    pub fn init(&mut self) {
        self.parse_ndk_libs();

        let architectures: Vec<String> = self.get_architectures();

        // If there's only one architecture and we haven't specified a default one..
        // then autochose what we have
        if architectures.len() == 1 && !self.has_default_architecture() {
            let arch = architectures[0].clone();
            let msg = format!("Only one architecture, autochoosing {}", arch);
            self.log(msg);
            self.set_default_architectures(architectures);
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_input(&self) -> String {
        self.input.clone()
    }

    pub fn get_default_architectures(&self) -> Vec<String> {
        self.default_architectures.clone()
    }

    pub fn set_default_architectures(&mut self, default_architectures: Vec<String>) {
        self.default_architectures = default_architectures;
    }

    // pub fn get_os(&self) -> String {
    //     self.os.clone()
    // }

    pub fn get_mk_path(&self) -> PathBuf {
        self.android_mk_path.clone()
    }

    pub fn get_preopt_dex(&self) -> bool {
        self.preopt_dex
    }

    pub fn set_preopt_dex(&mut self, dex: bool) {
        self.preopt_dex = dex;
    }

    pub fn extract_so(&self) -> bool {
        self.extract_so
    }
    pub fn set_extract_so(&mut self, should_extract_so: bool) {
        self.extract_so = should_extract_so;
    }

    pub fn privileged(&self) -> bool {
        self.privileged
    }

    pub fn set_privileged(&mut self, priv_app: bool) {
        self.privileged = priv_app
    }

    pub fn set_has_default_architecture(&mut self, has_def: bool) {
        self.has_default_architecture = has_def;
    }

    pub fn has_default_architecture(&self) -> bool {
        self.has_default_architecture
    }

    pub fn set_libraries(&mut self, libraries: Vec<String>) {
        self.libraries = libraries
    }

    pub fn set_architectures(&mut self, architectures: Vec<String>) {
        self.architectures = architectures
    }

    pub fn get_architectures(&self) -> Vec<String> {
        self.architectures.clone()
    }

    pub fn get_libraries(&self) -> Vec<String> {
        self.libraries.clone()
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn is_blueprint(&self) -> bool {
        self.blueprint
    }

    pub fn set_blueprint(&mut self, blueprint: bool) {
        self.blueprint = blueprint;
    }

    pub fn log<S>(&self, msg: S)
    where
        S: Into<String>,
    {
        if self.is_debug() {
            println!("{:?}", msg.into());
        }
    }

    pub fn parse_ndk_libs(&mut self) {
        let zip_files = zip::run(&self.get_input());
        let (arch, so): (Vec<String>, Vec<String>) = file::get_ndk_libs(zip_files);

        self.set_libraries(so);
        self.set_architectures(arch);
    }

    // TODO: return result
    pub fn gen_android_mk(&self) {
        if self.blueprint {
            file::gen_android_bp_con(self);
        } else {
            file::gen_android_mk_con(self);
        }
    }

    /// Entry point to generating Android.mk
    /// Creates a Androidmk struct from input which then allows you..
    /// to read architectures supported, .so libraries and more
    pub fn get_make_file_input() -> Androidmk {
        let matches = App::new("Generate Android make files automatically for APK's")
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
                .help("Android OS version to generate the mk (semver)"),
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
        .get_matches();

        // Input path of the apk, should never be empty!
        let input = matches.value_of("input").unwrap();
        // If empty, default to input
        // Get file name without path or ext
        let name_buf = file::file_name(input);
        let name = matches.value_of("name").unwrap_or(&name_buf);
        // Default architecture
        // If not supplied, it will add all architectures found in APK
        let default_architecture = matches.value_of("architecture").unwrap_or("");
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
        // Generate blueprint for soong instead of makefile
        let bp = matches.is_present("soong");

        let makefile = Androidmk::new(
            input,
            name,
            default_architecture,
            has_default_architecture,
            os,
            dex_opt,
            privileged,
            extract_so,
            debug,
            bp,
        );
        makefile
    }
}
