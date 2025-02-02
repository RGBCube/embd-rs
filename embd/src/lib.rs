#![allow(unexpected_cfgs)]
use std::{
    borrow::Cow,
    fs,
    path::Path,
};

#[cfg(not(procmacro2_semver_exempt))]
compile_error!(
    r#"pass `--cfg procmacro2_semver_exempt` to rustc to compile embd or add this to your `.cargo/config.toml`:

[build]
rustflags = [ "--cfg", "procmacro2_semver_exempt" ]

"#
);

#[doc(hidden)]
pub fn __string_runtime(neighbor: &str, path: &str) -> String {
    let file = Path::new(neighbor)
        .parent()
        .expect("Failed to get the parent of file")
        .join(path);

    fs::read_to_string(file).expect("Failed to read file")
}

/// Embed a files contents as a `&str` on release,
/// read from the filesystem as a `String` on debug.
///
/// # Example
///
/// ```
/// let content: Cow<'static, str> = embd::string!("main.rs");
/// ```
#[macro_export]
#[cfg(procmacro2_semver_exempt)]
macro_rules! string {
    ($path:literal) => {{
        #[cfg(debug_assertions)]
        {
            ::std::borrow::Cow::Owned::<'static, str>(::embd::__string_runtime(file!(), $path))
        }
        #[cfg(not(debug_assertions))]
        {
            ::std::borrow::Cow::Borrowed(include_str!($path))
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

/// Embed a files contents as a `&[u8]` on release,
/// read from the filesystem as a `Vec<u8>` on debug.
///
/// # Example
///
/// ```
/// fn main() {
///     // `assets/` is in the same directory as `src/`
///     let content: Cow<'static, [u8]> = embd::string!("../assets/icon.png");
/// }
/// ```
#[macro_export]
#[cfg(procmacro2_semver_exempt)]
macro_rules! bytes {
    ($path:literal) => {{
        #[cfg(debug_assertions)]
        {
            ::std::borrow::Cow::Owned::<'static, [u8]>(::embd::__bytes_runtime(file!(), $path))
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

impl DirEntry {
    /// Returns the absolute path of the entry.
    pub fn path(&self) -> &Cow<'_, str> {
        match self {
            DirEntry::File(file) => file.path(),
            DirEntry::Dir(dir) => dir.path(),
        }
    }
}

/// A directory.
#[derive(Debug, Clone)]
pub struct Dir {
    #[doc(hidden)]
    pub __children: Cow<'static, [DirEntry]>,
    #[doc(hidden)]
    pub __path: Cow<'static, str>, /* We are making it a &str because   *
                                    * include_*! takes a string anyway. */
}

impl Dir {
    /// Returns the children of the directory.
    pub fn children(&self) -> &Cow<'_, [DirEntry]> {
        &self.__children
    }

    /// Returns the absolute path of the directory.
    pub fn path(&self) -> &Cow<'_, str> {
        &self.__path
    }

    /// Collects all files from the directory into a vector.
    pub fn flatten(self) -> Vec<File> {
        let mut entries = Vec::new();

        for child in self.__children.into_owned() {
            // TODO: Eliminate allocation.
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
    #[doc(hidden)]
    pub __content: Cow<'static, [u8]>,
    #[doc(hidden)]
    pub __path: Cow<'static, str>,
}

impl File {
    /// Returns the content of the file.
    pub fn content(&self) -> &Cow<'_, [u8]> {
        &self.__content
    }

    /// Returns the absolute path of the file.
    pub fn path(&self) -> &Cow<'_, str> {
        &self.__path
    }
}

fn read_dir(directory: &Path) -> Vec<DirEntry> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(directory).expect("Failed to list directory contents") {
        let entry = entry.expect("Failed to read entry");

        let filetype = entry.file_type().expect("Failed to read entry filetype");

        let path = entry
            .path()
            .canonicalize()
            .expect("Failed to canonicalize path");

        let path_str = path
            .to_str()
            .expect("Failed to convert OsStr to str")
            .to_string();

        if filetype.is_dir() {
            let children = read_dir(&path);

            entries.push(DirEntry::Dir(Dir {
                __children: children.into(),
                __path: path_str.into(),
            }))
        } else if filetype.is_file() {
            let content = fs::read(&path).expect("Failed to read file contents");

            entries.push(DirEntry::File(File {
                __content: content.into(),
                __path: path_str.into(),
            }))
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

    let directory_str = directory
        .to_str()
        .expect("Failed to convert OsStr to str")
        .to_string();

    let children = read_dir(&directory);

    Dir {
        __children: children.into(),
        __path: directory_str.into(),
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
///     let content: embd::Dir = embd::dir!("../assets");
/// }
/// ```
#[cfg(procmacro2_semver_exempt)]
pub use embd_macros::__dir as dir;
