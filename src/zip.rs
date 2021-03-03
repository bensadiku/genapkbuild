use std::fs::File;
use std::io::{Read, Seek};
use zip::read::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipResult;

pub fn run(path: &str) -> Vec<String> {
    let mut file = File::open(path).expect(&format!("Couldn't open file {}", &path));
    let files = browse_zip_archive(&mut file, |f| Ok(format!("{}", f.name())));

    match files {
        Ok(file) => {
            // println!("{:?}", file);
            file
        }
        Err(e) => {
            panic!(" Panic {:?}", e);
        }
    }
    // println!("{:?}", files);
}

fn browse_zip_archive<T, F, U>(buf: &mut T, browse_func: F) -> ZipResult<Vec<U>>
where
    T: Read + Seek,
    F: Fn(&ZipFile) -> ZipResult<U>,
{
    let mut archive = ZipArchive::new(buf)?;
    (0..archive.len())
        .map(|i| archive.by_index(i).and_then(|file| browse_func(&file)))
        .collect()
}
