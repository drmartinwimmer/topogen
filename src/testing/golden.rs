use std::{
    env,
    fs::{File, create_dir_all},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use tempfile::{Builder, NamedTempFile};

enum WritePath {
    TempFile(NamedTempFile),
    UpdateGolden,
}

pub struct Golden {
    golden_path: PathBuf,
    write_path: WritePath,
}

impl Golden {
    pub fn new(path: &Path) -> Self {
        let golden_path = PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .join("testdata/golden")
            .join(path);
        let write_path = match Self::is_update() {
            true => WritePath::UpdateGolden,
            false => WritePath::TempFile(match path.file_name() {
                Some(name) => Builder::new()
                    .suffix(name)
                    .tempfile()
                    .expect("failed to create suffixed temporary golden file"),
                None => Builder::new()
                    .tempfile()
                    .expect("failed to create temporary golden file"),
            }),
        };
        if matches!(&write_path, &WritePath::UpdateGolden) {
            create_dir_all(
                golden_path
                    .parent()
                    .expect("golden file path must have a parent directory"),
            )
            .expect("failed to create parent directory for golden file");
        }

        Self {
            golden_path,
            write_path,
        }
    }

    fn is_update() -> bool {
        matches!(env::var("UPDATE_GOLDEN"),
            Ok(str) if str == "true" || str == "1")
    }

    pub fn write_path(&self) -> &Path {
        match self.write_path {
            WritePath::TempFile(ref tempfile) => tempfile.path(),
            WritePath::UpdateGolden => self.golden_path(),
        }
    }

    pub fn golden_path(&self) -> &Path {
        &self.golden_path
    }

    pub fn is_identical(&self) -> bool {
        if self.golden_path() == self.write_path() {
            return true;
        }
        let Ok(golden_file) = File::open(self.golden_path()) else {
            return false;
        };
        let mut golden = BufReader::new(golden_file);
        let Ok(written_file) = File::open(self.write_path()) else {
            return false;
        };
        let mut written = BufReader::new(written_file);
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
