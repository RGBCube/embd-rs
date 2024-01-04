#![cfg(procmacro2_semver_exempt)]

use std::{
    fs,
    path::PathBuf,
};

use proc_macro as pm1;
use proc_macro2::TokenStream;
use quote::{
    quote,
    ToTokens,
};
use syn::{
    parse_macro_input,
    spanned::Spanned,
    LitStr,
};

struct PathBuf2(PathBuf);

impl ToTokens for PathBuf2 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let raw = self
            .0
            .to_str()
            .expect("Failed to get the string representation of PathBuf");

        tokens.extend(quote! {
            ::std::path::PathBuf::from(#raw)
        });
    }
}

struct TokenVec(Vec<TokenStream>);

impl ToTokens for TokenVec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = &self.0;

        tokens.extend(quote! {
            vec![#(#inner),*]
        });
    }
}

#[proc_macro]
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

fn dir_debug(path: &str) -> TokenStream {
    quote! {
        ::embed::__dir_runtime(file!(), #path)
    }
}

fn dir_release(input: TokenStream, path: &str) -> TokenStream {
    let neighbor = TokenStream::from(input).span().source_file().path();

    let base = neighbor.parent().expect("Failed to get the parent of file");

    let directory = PathBuf2(
        base.join(path)
            .canonicalize()
            .expect("Failed to canonicalize path"),
    );

    let children = read_dir(&directory.0);

    quote! {
        ::embed::Dir {
            children: #children,
            path: #directory,
        }
    }
}

fn read_dir(directory: &PathBuf) -> TokenVec {
    let mut entries = Vec::new();

    for entry in fs::read_dir(directory).expect("Failed to list directory contents") {
        let entry = entry.expect("Failed to read entry");

        let path = PathBuf2(entry.path());

        let filetype = fs::metadata(&path.0)
            .expect("Failed to get file metadata")
            .file_type();

        if filetype.is_dir() {
            let children = read_dir(&path.0);

            entries.push(quote! {
                ::embed::DirEntry::Dir(::embed::Dir {
                    children: #children,
                    path: #path,
                })
            });
        } else if filetype.is_file() {
            let path_str = path
                .0
                .to_str()
                .expect("Failed to get the string representation of PathBuf");

            entries.push(quote! {
                ::embed::DirEntry::File(::embed::File {
                    content: ::std::borrow::Cow::Borrowed(include_bytes!(#path_str)),
                    path: #path,
                })
            });
        }
    }

    TokenVec(entries)
}
