#[macro_export]
macro_rules! string {
    ($path:literal) => {{
        use std::borrow::Cow;

        #[cfg(debug_assertions)]
        {
            use std::{
                fs,
                path::Path,
            };

            let file = Path::new(file!())
                .parent()
                .expect("embed: file has no parent")
                .join($path);

            Cow::<'static, str>::Owned(fs::read_to_string(&file).unwrap_or_else(|error| {
                panic!(
                    "embed: failed to read file {file}: {error}",
                    file = file.display()
                )
            }))
        }
        #[cfg(not(debug_assertions))]
        {
            Cow::Borrowed(include_str!($path))
        }
    }};
}

#[macro_export]
macro_rules! bytes {
    ($path:literal) => {{
        use std::borrow::Cow;

        #[cfg(debug_assertions)]
        {
            use std::{
                fs,
                path::Path,
            };

            let file = Path::new(file!())
                .parent()
                .expect("embed: file has no parent")
                .join($path);

            Cow::<'static, str>::Owned(fs::read_to_string(&file).unwrap_or_else(|error| {
                panic!(
                    "embed: failed to read file {file}: {error}",
                    file = file.display()
                )
            }))
        }
        #[cfg(not(debug_assertions))]
        {
            Cow::Borrowed(include_bytes!($path))
        }
    }};
}

#[allow(dead_code)]
fn check_validity() {
    string!("test.txt");
    bytes!("test.txt");
}
