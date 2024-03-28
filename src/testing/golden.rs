use std::{
    env,
    fs::{create_dir_all, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use tempdir::TempDir;

pub struct Golden {
    golden_path: PathBuf,
    write_path: PathBuf,
    _tempdir: TempDir,
}

impl Golden {
    pub fn new(path: &Path) -> Self {
        let golden_path = PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .join("testdata/golden")
            .join(path);
        let tempdir = TempDir::new("golden").unwrap();
        let write_path = match env::var("UPDATE_GOLDEN") {
            Ok(str) if str == "true" || str == "1" => PathBuf::new().join(&golden_path),
            _ => PathBuf::new().join(tempdir.path()).join(path),
        };
        println!("write_path: {}", write_path.display());
        create_dir_all(write_path.parent().unwrap()).unwrap();

        Self {
            golden_path,
            write_path,
            _tempdir: tempdir,
        }
    }

    pub fn write_path(&self) -> &Path {
        &self.write_path
    }

    pub fn golden_path(&self) -> &Path {
        &self.golden_path
    }

    pub fn is_identical(&self) -> bool {
        if self.golden_path() == self.write_path() {
            return true;
        }
        let mut golden = BufReader::new(match File::open(self.golden_path()) {
            Ok(file) => file,
            Err(_) => return false,
        });
        let mut written = BufReader::new(match File::open(self.write_path()) {
            Ok(file) => file,
            Err(_) => return false,
        });
        loop {
            let read = match (golden.fill_buf(), written.fill_buf()) {
                (Ok(golden_buf), Ok(written_buf)) if golden_buf == written_buf => {
                    if golden_buf.is_empty() {
                        return true;
                    }
                    golden_buf.len()
                }
                _ => return false,
            };
            golden.consume(read);
            written.consume(read);
        }
    }
}
