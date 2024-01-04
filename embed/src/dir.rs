use std::{
    fs::{
        self,
    },
    path::PathBuf,
};

pub enum DirEntry {
    Dir(Dir),
    File(File),
}

pub struct Dir {
    pub children: Vec<DirEntry>,
    pub path: PathBuf,
}

impl Dir {
    pub fn flatten(self) -> Vec<File> {
        let mut entries = Vec::new();

        for child in self.children {
            match child {
                DirEntry::File(file) => entries.push(file),
                DirEntry::Dir(dir) => entries.append(&mut dir.flatten()),
            }
        }

        entries
    }
}

pub struct File {
    pub content: Vec<u8>,
    pub path: PathBuf,
}

fn read_dir(path: &PathBuf) -> Vec<DirEntry> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path).expect("Failed to list directory contents") {
        let entry = entry.expect("Failed to read entry");

        let filetype = entry.file_type().expect("Failed to read entry filetype");
        let path = entry.path();

        if filetype.is_dir() {
            let children = read_dir(&path);

            entries.push(DirEntry::Dir(Dir { children, path }))
        } else if filetype.is_file() {
            let content = fs::read(&path).expect("Failed to read file contents");

            entries.push(DirEntry::File(File { content, path }))
        }
    }

    entries
}

pub fn __include_dir(caller: &str, path: &str) -> Dir {
    let path = PathBuf::from(caller)
        .parent()
        .expect("Failed to get the parent of file")
        .join(path);

    let children = read_dir(&path);

    Dir { children, path }
}

#[macro_export]
macro_rules! __dir {
    ($caller:literal, $path:literal) => {{
        #[cfg(debug_assertions)]
        {
            ::embed::__include_dir($caller, $path)
        }
        #[cfg(not(debug_assertions))]
        {
            ::embed_macros::__include_dir!($caller, $path)
        }
    }};
}

#[macro_export]
macro_rules! dir {
    ($path:literal) => {
        ::embed::__dir!(file!(), $path)
    };
}
