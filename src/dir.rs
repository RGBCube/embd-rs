use std::{
    fs,
    path::PathBuf,
    time::SystemTime,
};

use include_dir::{
    Dir,
    DirEntry,
    File,
};

fn read_dir(dir: &PathBuf) -> Vec<PathBuf> {
    if !dir.is_dir() {
        panic!("embed: {path} is not a directory", path = dir.display());
    }

    let mut paths = Vec::new();

    for entry in dir.read_dir().unwrap_or_else(|error| {
        panic!(
            "embed: failed to read directory {dir}: {error}",
            dir = dir.display()
        )
    }) {
        paths.push(
            entry
                .unwrap_or_else(|error| panic!("embed: failed to resolve entry: {error}"))
                .path(),
        );
    }

    paths.sort();
    paths
}

fn file_to_entry<'a>(path: &'a PathBuf) -> DirEntry<'a> {
    let abs = path.canonicalize().unwrap_or_else(|error| {
        panic!(
            "embed: failed to resolve path {path}: {error}",
            path = path.display()
        )
    });

    let contents = fs::read(&path).unwrap_or_else(|error| {
        panic!(
            "embed: failed to read file {path}: {error}",
            path = path.display()
        )
    });

    let mut entry = File::new(abs.to_str().unwrap(), contents.as_slice());

    if let Ok(metadata) = path.metadata() {
        entry = entry.with_metadata(include_dir::Metadata::new(
            metadata
                .accessed()
                .unwrap_or_else(|error| {
                    panic!("embed: failed to read metadata.accessed of {metadata:?}: {error}");
                })
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|error| {
                    panic!("embed: failed to calculate time difference: {error}")
                }),
            metadata
                .created()
                .unwrap_or_else(|error| {
                    panic!("embed: failed to read metadata.created of {metadata:?}: {error}");
                })
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|error| {
                    panic!("embed: failed to calculate time difference: {error}")
                }),
            metadata
                .modified()
                .unwrap_or_else(|error| {
                    panic!("embed: failed to read metadata.modified of {metadata:?}: {error}");
                })
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|error| {
                    panic!("embed: failed to calculate time difference: {error}")
                }),
        ));
    }

    DirEntry::File(entry)
}

fn dir_to_entries<'a>(root: &'a PathBuf, path: &'a PathBuf) -> Vec<DirEntry<'a>> {
    let mut entries = Vec::new();

    for child in read_dir(path) {
        if child.is_dir() {
            entries.push(DirEntry::Dir(Dir::new(
                root.as_os_str().to_str().unwrap_or_else(|| {
                    panic!(
                        "embed: failed to convert {lossy} to &str as it was not valid unicode",
                        lossy = root.to_string_lossy()
                    )
                }),
                &dir_to_entries(root, &child),
            )));
        }
        else if child.is_file() {
            entries.push(file_to_entry(&child))
        }
        else {
            panic!(
                "{child} is neither a file or directory",
                child = child.display()
            )
        }
    }

    entries
}

fn include_dir<'a>(path_str: &'a str) -> Dir<'a> {
    let path = PathBuf::from(path_str);

    let entries = dir_to_entries(&path, &path);

    Dir::new(path_str, entries.as_slice())
}

#[macro_export]
macro_rules! dir {
    ($path:literal) => {{
        #[cfg(debug_assertions)]
        {
            include_dir($path)
        }
        #[cfg(not(debug_assertions))]
        {
            include_dir!($path)
        }
    }};
}
