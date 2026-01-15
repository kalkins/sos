use log::info;
use uefi::{
    fs::{FileSystem, Path},
    prelude::*,
};

pub fn enumerate_files(fs: &mut FileSystem, path: impl AsRef<Path>) {
    let path = path.as_ref();
    let mut dir_path = path.to_path_buf();

    if dir_path.is_empty() {
        dir_path.push(cstr16!("/"));
    }

    let metadata = match fs.metadata(&dir_path) {
        Ok(m) => m,
        Err(error) => {
            info!("Could not read metadata for path '{dir_path}': {error}");
            return;
        }
    };

    if metadata.is_directory() {
        info!("Found directory '{path}'");
        let files = match fs.read_dir(&dir_path) {
            Ok(f) => f,
            Err(error) => {
                info!("Could not read children of dir '{path}': {error}");
                return;
            }
        };

        for file in files {
            match file {
                Ok(file) => {
                    let file_name = file.file_name();
                    if file_name == cstr16!(".") || file_name == cstr16!("..") {
                        continue;
                    }

                    let mut file_path = path.to_path_buf();
                    file_path.push(file_name);
                    enumerate_files(fs, file_path);
                }
                Err(error) => info!("Could not access file: {error}"),
            }
        }
    } else {
        info!("Found file '{path}'");
    }
}
