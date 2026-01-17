use alloc::vec::Vec;
use log::info;
use uefi::{
    fs::{FileSystem, Path},
    prelude::*,
    proto::media::fs::SimpleFileSystem,
};

pub fn get_file_systems_with_file(path: impl AsRef<Path>) -> Result<Vec<FileSystem>, uefi::Error> {
    get_file_systems().map(|file_systems| {
        file_systems
            .into_iter()
            .filter_map(|mut fs| match file_exists(&mut fs, &path) {
                Ok(true) => Some(fs),
                Ok(false) => None,
                Err(error) => {
                    info!("Could not inspect file {}: {error}", path.as_ref());
                    None
                }
            })
            .collect()
    })
}

pub fn get_file_systems() -> Result<Vec<FileSystem>, uefi::Error> {
    let handles = boot::find_handles::<SimpleFileSystem>()?;

    let protocols = handles
        .iter()
        .map(|h| boot::open_protocol_exclusive::<SimpleFileSystem>(*h))
        .filter_map(|r| {
            r.inspect_err(|error| {
                info!("Could not retrieve SimpleFileSystem protocol for handle: {error}")
            })
            .ok()
        })
        .map(|proto| FileSystem::new(proto))
        .collect();

    Ok(protocols)
}

pub fn file_exists(fs: &mut FileSystem, path: impl AsRef<Path>) -> Result<bool, uefi::fs::Error> {
    fs.try_exists(&path).and_then(|exists| match exists {
        true => fs
            .metadata(&path)
            .map(|metadata| metadata.is_regular_file()),
        false => Ok(false),
    })
}

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
