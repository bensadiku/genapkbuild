mod build;
mod cli;
mod file;
mod utils;
mod makefile;
mod blueprint;
mod zip;

use build::BuildSystemBase;
use cli::read_input;

// TODO, use Result here since it maps to (EXIT_SUCCESS,EXIT_FAILURE) anyway
fn main() {
    let base: BuildSystemBase = read_input();
    let exit_code = base.generate();

    std::process::exit(exit_code);
}
