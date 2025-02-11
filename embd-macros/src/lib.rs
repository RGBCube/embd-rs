#![allow(unexpected_cfgs)]
#[cfg(procmacro2_semver_exempt)]
use std::{
    fs,
    path::Path,
};

#[cfg(not(procmacro2_semver_exempt))]
compile_error!(
    r#"pass `--cfg procmacro2_semver_exempt` to rustc to compile embd-macros or add this to your `.cargo/config.toml`:

[build]
rustflags = [ "--cfg", "procmacro2_semver_exempt" ]

"#
);

#[cfg(procmacro2_semver_exempt)]
use proc_macro as pm1;
use proc_macro2::TokenStream;
use quote::{
    quote,
    ToTokens,
};
#[cfg(procmacro2_semver_exempt)]
use syn::{
    parse_macro_input,
    spanned::Spanned,
    LitStr,
};

struct TokenVec(Vec<TokenStream>);

impl ToTokens for TokenVec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = &self.0;

        tokens.extend(quote! {
            ::std::borrow::Cow::Borrowed(&[#(#inner),*])
        });
    }
}

#[proc_macro]
#[cfg(procmacro2_semver_exempt)]
pub fn __dir(input: pm1::TokenStream) -> pm1::TokenStream {
    let input2 = input.clone();
    let path = parse_macro_input!(input2 as LitStr).value();

    let debug = dir_debug(&path);
    let release = dir_release(input.into(), &path);

    quote! {
        {
            #[cfg(debug_assertions)]
            {
                #debug
            }
            #[cfg(not(debug_assertions))]
            {
                #release
            }
        }
    }
    .into()
}

#[cfg(procmacro2_semver_exempt)]
fn dir_debug(path: &str) -> TokenStream {
    quote! {
        ::embd::__dir_runtime(file!(), #path)
    }
}

#[cfg(procmacro2_semver_exempt)]
fn dir_release(input: TokenStream, path: &str) -> TokenStream {
    let neighbor = input.span().source_file().path();

    let base = neighbor.parent().expect("Failed to get the parent of file");

    let directory = base.join(path).canonicalize().expect("Failed to canonicalize path");

    let directory_str = directory.to_str().expect("Failed to convert OsStr to str");

    let children = read_dir(&directory);

    quote! {
        ::embd::Dir {
            __children: #children,
            __path: ::std::borrow::Cow::Borrowed(#directory_str),
        }
    }
}

#[cfg(procmacro2_semver_exempt)]
fn read_dir(directory: &Path) -> TokenVec {
    let mut entries = Vec::new();

    for entry in fs::read_dir(directory).expect("Failed to list directory contents") {
        let entry = entry.expect("Failed to read entry");

        let path = entry.path();

        let path_str = path
            .to_str()
            .expect("Failed to get the string representation of PathBuf");

        let filetype = fs::metadata(&path).expect("Failed to get file metadata").file_type();

        if filetype.is_dir() {
            let children = read_dir(&path);

            entries.push(quote! {
                ::embd::DirEntry::Dir(::embd::Dir {
                    __children: #children,
                    __path: ::std::borrow::Cow::Borrowed(#path_str),
                })
            });
        } else if filetype.is_file() {
            entries.push(quote! {
                ::embd::DirEntry::File(::embd::File {
                    __content: ::std::borrow::Cow::Borrowed(include_bytes!(#path_str)),
                    __path: ::std::borrow::Cow::Borrowed(#path_str),
                })
            });
        }
    }

    TokenVec(entries)
}
