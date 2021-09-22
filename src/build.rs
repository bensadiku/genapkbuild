use super::blueprint::BluePrint;
use super::file;
use super::makefile::MakeFile;
use super::utils;
use super::zip;

pub trait BuildSystem {
    fn generate(&self) -> i32;
}

#[derive(Debug, Clone)]
pub struct BuildSystemBase {
    /// The path of the apk
    input: String,
    /// Name of the apk
    name: String,
    /// Override architectures
    default_architectures: Vec<String>,
    /// Has default architectures overwritten
    has_default_architecture: bool,
    /// OS version (Unused remove?)
    os: String,
    /// To enable or disable pre-optimization
    preopt_dex: bool,
    /// Make an app privileged (priv-app)
    privileged: bool,
    /// JNI libs found on apk
    libraries: Vec<String>,
    /// Architectures found on apk
    architectures: Vec<String>,
    /// Extract JNI libs if any,
    /// Only available for makefiles
    extract_so: bool,
    /// Full logs
    debug: bool,
    /// Output a Android.bp file
    blueprint: bool,
    /// Output a Android.mk file
    makefile: bool,
    /// Output a bazel BUILD file (NOT IMPLEMENTED)
    bazel: bool,
}
impl BuildSystemBase {
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
        mk: bool,
        bz: bool,
    ) -> Self
    where
        I: Into<String>,
        N: Into<String>,
        D: Into<String>,
        O: Into<String>,
    {
        let default_architectures = utils::input_to_abi_vec(&default.into());

        let mut base = BuildSystemBase {
            input: input.into(),
            name: name.into(),
            default_architectures: default_architectures,
            has_default_architecture: has_default,
            os: os.into(),
            preopt_dex: preopt_dex,
            privileged: privileged,
            libraries: Vec::new(),
            architectures: Vec::new(),
            extract_so: extract_so,
            debug: debug,
            blueprint: bp,
            makefile: mk,
            bazel: bz,
        };
        base.init();
        base
    }
    /// Handles initialization of some variable
    /// Reads apk files, extracts .so files and adds them as list
    /// Reads architectures, extracts them as list
    pub fn init(&mut self) {
        self.parse_ndk_libs();

        let architectures: Vec<String> = self.get_architectures();

        // If there's only one architecture and we haven't specified a default one..
        // then autochose what we have
        if architectures.len() == 1 && !self.has_default_architecture {
            let arch = architectures[0].clone();
            let msg = format!("Only one architecture, autochoosing {}", arch);
            self.log(msg);
            self.set_default_architectures(architectures);
        }
        // If there's no name passed, override name with apk name
        // If there's one passed, respect that
        let input = self.get_input();
        let name = self.get_name();
        let default_name = if name.is_empty() {
            let name_buf = file::file_name(&input);
            name_buf
        } else {
            name
        };
        self.set_name(default_name);
    }
    pub fn parse_ndk_libs(&mut self) {
        let zip_files = zip::run(&self.input);
        let (arch, so): (Vec<String>, Vec<String>) = file::get_ndk_libs(zip_files);

        self.set_libraries(so);
        self.set_architectures(arch);
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_name<N>(&mut self, name: N) -> &mut Self
    where
        N: Into<String>,
    {
        self.name = name.into();
        self
    }
    pub fn get_input(&self) -> String {
        self.input.clone()
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
    pub fn get_default_architectures(&self) -> Vec<String> {
        self.default_architectures.clone()
    }

    pub fn set_default_architectures(&mut self, default_architectures: Vec<String>) {
        self.default_architectures = default_architectures;
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

    pub fn get_libraries(&self) -> Vec<String> {
        self.libraries.clone()
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    fn new_mk(&self) -> MakeFile {
        MakeFile {
            build_system: self.clone(),
        }
    }
    fn new_bp(&self) -> BluePrint {
        BluePrint {
            build_system: self.clone(),
        }
    }
    fn is_blueprint(&self) -> bool {
        self.blueprint
    }
    fn is_make_file(&self) -> bool {
        self.makefile
    }
    pub fn log<S>(&self, msg: S)
    where
        S: Into<String>,
    {
        if self.debug {
            println!("{:?}", msg.into());
        }
    }
    pub fn generate(&self) -> i32 {
        if self.is_make_file() {
            let make = self.new_mk();
            self.gen(make)
        } else if self.is_blueprint() {
            let bp = self.new_bp();
            self.gen(bp)
        } else {
            panic!("Not implemented!");
        }
    }

    fn gen<T>(&self, system: T) -> i32
    where
        T: BuildSystem,
    {
        system.generate()
    }
}

impl Default for BuildSystemBase {
    fn default() -> BuildSystemBase {
        BuildSystemBase {
            input: "tests/data/multipleArch.apk".into(),     // input
            name: "multipleArch".into(),                     // name
            default_architectures: vec!["arm64-v8a".into()], // default_architecture
            has_default_architecture: false,                 // has default architecture
            os: "6.0".into(),                                // (un-used) os version
            preopt_dex: false,                               // pre-optimize dex files
            privileged: false,                               // priviledged
            libraries: Vec::new(),                           // JNI libraries found within APK
            architectures: Vec::new(),                       // architectures found within APK
            extract_so: false,                               // extract_so
            debug: true,                                     // debug flag
            blueprint: false,                                // generate blueprint Android.bp
            makefile: true,                                  // generate makefile Android.mk
            bazel: false,                                    // generate bazel BUILD
        }
    }
}

pub struct BuildSystemBaseBuilder {
    base: BuildSystemBase,
}

impl BuildSystemBaseBuilder {
    pub fn new() -> Self {
        BuildSystemBaseBuilder {
            base: BuildSystemBase::default(),
        }
    }
    pub fn set_input<N>(&mut self, input: N) -> &mut Self
    where
        N: Into<String>,
    {
        self.base.input = input.into();
        self
    }
    pub fn set_name<N>(&mut self, name: N) -> &mut Self
    where
        N: Into<String>,
    {
        self.base.name = name.into();
        self
    }
    pub fn set_make_file(&mut self, is_make_file: bool) -> &mut Self {
        self.base.makefile = is_make_file;
        self
    }
    pub fn set_blueprint(&mut self, bp: bool) -> &mut Self {
        self.base.blueprint = bp;
        self
    }
    pub fn set_extract_so(&mut self, so: bool) -> &mut Self {
        self.base.extract_so = so;
        self
    }
    pub fn set_default_architectures(&mut self, default_architectures: Vec<String>) {
        self.base.default_architectures = default_architectures;
    }
    pub fn get_default_architectures(&self) -> Vec<String> {
        self.base.get_default_architectures()
    }
    pub fn set_has_default_architecture(&mut self, has_def: bool) {
        self.base.has_default_architecture = has_def;
    }
    pub fn has_default_architecture(&self) -> bool {
        self.base.has_default_architecture
    }
    // The way input receives it, as a string
    pub fn override_arch(&mut self, arch: String) {
        let default_architectures = utils::input_to_abi_vec(&arch);
        self.set_default_architectures(default_architectures);
        self.set_has_default_architecture(true);
    }
    pub fn build(&mut self) -> BuildSystemBase {
        self.base.init();
        self.base.clone()
    }
}
