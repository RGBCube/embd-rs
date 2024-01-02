use std::{
    fs,
    path::PathBuf,
};

use proc_macro as pm1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    LitStr,
};

#[proc_macro]
pub fn __include_dir(tokens: pm1::TokenStream) -> pm1::TokenStream {
    let path = parse_macro_input!(tokens as LitStr).value();

    let path = PathBuf::from(path)
        .canonicalize()
        .expect("Failed to get the canonical path of the DirEntry");

    let path_str = path
        .to_str()
        .expect("Failed to get the string representation of PathBuf");

    let children = read_dir(&path);
    let children_tokens = quote! {
        vec![#(#children),*]
    };

    (quote! {
        ::embed::Dir {
            children: #children_tokens,
            path: ::std::path::PathBuf::from(#path_str),
        }
    })
    .into()
}

fn read_dir(path: &PathBuf) -> Vec<TokenStream> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path).expect("Failed to list directory contents") {
        let entry = entry.expect("Failed to read entry");

        let path = entry
            .path()
            .canonicalize()
            .expect("Failed to get the canonical path of the DirEntry");

        let path_str = path
            .to_str()
            .expect("Failed to get the string representation of PathBuf");

        let filetype = fs::metadata(&path)
            .expect("Failed to get file metadata")
            .file_type();

        if filetype.is_dir() {
            let children = read_dir(&path);
            let children_tokens = quote! {
                vec![#(#children),*]
            };

            entries.push(quote! {
                ::embed::DirEntry(::embed::Dir {
                    children: #children_tokens,
                    path: ::std::path::PathBuf::from(#path_str),
                })
            });
        } else if filetype.is_file() {
            entries.push(quote! {
                ::embed::DirEntry(::embed::File {
                    content: ::include_bytes!(#path_str),
                    path: ::std::path::PathBuf::from(#path_str),
                })
            });
        }
    }

    entries
}
