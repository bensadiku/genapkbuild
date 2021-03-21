use super::makefile::Androidmk;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Seek};
use std::path::Path;
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

pub fn extract_zip(mk: &Androidmk) {
    let default_architecture = mk.get_default_architecture();
    let input = &mk.get_input();
    let fname = std::path::Path::new(input);
    mk.log(format!(
        "Extracting APK: {:?} for architecture {} ",
        fname, default_architecture
    ));

    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                mk.log(format!("File {} comment: {}", i, comment));
            }
        }

        if (&*file.name()).ends_with('/') {
            mk.log(format!("File {} extracted to \"{}\"", i, outpath.display()));
            fs::create_dir_all(&outpath).unwrap();
        } else if (&*file.name()).ends_with(".so") {
            // Create directory if it does not exist and if the architecture matches
            // TODO: Panic if there's multiple abis and the default abi doesn't match any of them
            if let Some(p) = outpath.parent() {
                let directory_arch = p.file_name().unwrap().to_str().unwrap();
                if directory_arch == default_architecture {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                    let mut outfile = fs::File::create(&outpath).unwrap();
                    io::copy(&mut file, &mut outfile).unwrap();
                    mk.log(format!(
                        "File {} extracted to \"{}\" ({} bytes)",
                        i,
                        outpath.display(),
                        file.size()
                    ));
                } else {
                    continue;
                }
            }
        } else {
            continue;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}
