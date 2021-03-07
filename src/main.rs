mod file;
mod makefile;
mod tests;
mod zip;

use makefile::Androidmk;

fn main() {
    let mk = Androidmk::get_make_file_input();
  
    let _ret2 = mk.gen_android_mk();
}
