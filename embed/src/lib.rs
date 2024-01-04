#![cfg(procmacro2_semver_exempt)]

use std::{
    borrow::Cow,
    fs,
    path::Path,
};

#[doc(hidden)]
pub fn __string_runtime(neighbor: &str, path: &str) -> String {
    let file = Path::new(neighbor)
        .parent()
        .expect("Failed to get the parent of file")
        .join(path);

    fs::read_to_string(file).expect("Failed to read file")
}

/// Embed a files contents as a &str on release,
/// read from the filesystem as a String on debug.
///
/// # Example
///
/// ```
/// fn main() {
///     let content = embed::string!("main.rs");
/// }
/// ```
#[macro_export]
macro_rules! string {
    ($path:literal) => {{
        use ::std::borrow::Cow;

        #[cfg(debug_assertions)]
        {
            Cow::Owned(::embed::__string_runtime(file!(), $path))
        }
        #[cfg(not(debug_assertions))]
        {
            Cow::Borrowed(include_str!($path))
        }
    }};
}

#[doc(hidden)]
pub fn __bytes_runtime(neighbor: &str, path: &str) -> Vec<u8> {
    let file = Path::new(neighbor)
        .parent()
        .expect("Failed to get the parent of file")
        .join(path);

    fs::read(file).expect("Failed to read file")
}

/// Embed a files contents as a &[u8] on release,
/// read from the filesystem as a Vec<u8> on debug.
///
/// # Example
///
/// ```
/// fn main() {
///     // `assets/` is in the same directory as `src/`
///     let content = embed::string!("../assets/icon.png");
/// }
/// ```
#[macro_export]
macro_rules! bytes {
    ($path:literal) => {{
        #[cfg(debug_assertions)]
        {
            ::std::borrow::Cow::Owned(::embed::__bytes_runtime(file!(), $path))
        }
        #[cfg(not(debug_assertions))]
        {
            ::std::borrow::Cow::Borrowed(include_bytes!($path))
        }
    }};
}

/// A directory entry.
#[derive(Debug, Clone)]
pub enum DirEntry {
    /// A directory.
    Dir(Dir),
    /// A file.
    File(File),
}

/// A directory.
#[derive(Debug, Clone)]
pub struct Dir {
    /// The entries the directory houses.
    pub children: Vec<DirEntry>,
    /// The absolute path of the directory.
    pub path: Cow<'static, Path>,
}

impl Dir {
    /// Collects all files from the directory into a vector.
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

/// A file.
#[derive(Debug, Clone)]
pub struct File {
    /// The content of the file in bytes.
    pub content: Cow<'static, [u8]>,
    /// The absolute path of the file.
    pub path: Cow<'static, Path>,
}

fn read_dir(directory: &Path) -> Vec<DirEntry> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(directory).expect("Failed to list directory contents") {
        let entry = entry.expect("Failed to read entry");

        let filetype = entry.file_type().expect("Failed to read entry filetype");

        let path = Cow::Owned::<'static, Path>(
            entry
                .path()
                .canonicalize()
                .expect("Failed to canonicalize path"),
        );

        if filetype.is_dir() {
            let children = read_dir(path.as_ref());

            entries.push(DirEntry::Dir(Dir { children, path }))
        } else if filetype.is_file() {
            let content = Cow::Owned(fs::read(&path).expect("Failed to read file contents"));

            entries.push(DirEntry::File(File { content, path }))
        }
    }

    entries
}

#[doc(hidden)]
pub fn __dir_runtime(neighbor: &str, path: &str) -> Dir {
    let directory = Path::new(neighbor)
        .parent()
        .expect("Failed to get the parent of file")
        .join(path)
        .canonicalize()
        .expect("Failed to canonicalize path");

    let children = read_dir(&directory);

    Dir {
        children,
        path: Cow::Owned(directory),
    }
}

/// Embed a directories contents.
/// The content value of File will be Borrowed on release,
/// and Owned on debug.
///
/// # Example
///
/// ```
/// fn main() {
///     let content = embed::dir!("../assets");
/// }
/// ```
pub use embed_macros::__dir as dir;
