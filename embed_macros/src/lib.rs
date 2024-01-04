#![cfg(procmacro2_semver_exempt)]

use std::{
    env,
    fs,
    path::PathBuf,
};

use proc_macro::{
    self as pm1,
    TokenStream,
};
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

impl ToTokens for PathBuf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let raw = self
            .as_str()
            .expect("Failed to get the string representation of PathBuf");

        tokens.append(quote! {
            ::std::path::PathBuf::from(#raw)
        });
    }
}

impl ToTokens for Vec<TokenStream> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(quote! {
            vec![#(#self),*]
        });
    }
}

pub fn dir_debug(input: pm1::TokenStream) -> pm1::TokenStream {
    let path = parse_macro_input!(input as LitStr).value();

    quote! {
        ::embed::__include_dir_runtime(file!(), #path)
    };
}

pub fn dir_release(input: pm1::TokenStream) -> pm1::TokenStream {
    let neigbor = TokenStream::from(input.clone()).span().source_file().path();
    let path = parse_macro_input!(input as LitStr).value();

    let base = neighbor.parent().expect("Failed to get the parent of file");

    let directory = base
        .join(path)
        .canonicalize()
        .expect("Failed to canonicalize path");

    let children = read_dir(&directory);

    quote! {
        ::embed::Dir {
            children: #children,
            path: #directory,
        }
    }
    .into()
}

fn read_dir(directory: &PathBuf) -> Vec<DirEntry> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path).expect("Failed to list directory contents") {
        let entry = entry.expect("Failed to read entry");

        let path = entry.path();

        let filetype = fs::metadata(&path)
            .expect("Failed to get file metadata")
            .file_type();

        if filetype.is_dir() {
            let children = read_dir(&path);

            entries.push(quote! {
                ::embed::DirEntry::Dir(::embed::Dir {
                    children: #children,
                    path: #path,
                })
            });
        } else if filetype.is_file() {
            entries.push(quote! {
                ::embed::DirEntry::File(::embed::File {
                    content: ::std::borrow::Cow::Borrowed(include_bytes!(#path_str)),
                    path: #path,
                })
            });
        }
    }

    entries
}
