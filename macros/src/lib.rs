#![cfg(procmacro2_semver_exempt)]

use std::{
    fs,
    path::PathBuf,
};

use proc_macro as pm1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    spanned::Spanned,
    LitStr,
};

#[proc_macro]
pub fn __include_dir(input: pm1::TokenStream) -> pm1::TokenStream {
    let caller = TokenStream::from(input).span().source_file().path();

    let input2 = input.clone();
    let path = parse_macro_input!(input2 as LitStr).value();

    let path = caller
        .parent()
        .expect("Failed to get the parent of file")
        .join(path);

    let path = if path.ends_with("..") {
        path.parent().unwrap().to_path_buf()
    } else {
        path
    };

    let path_str = path
        .to_str()
        .expect("Failed to get the string representation of PathBuf");

    if path_str.ends_with(".") {
        return syn::Error::new_spanned(
            TokenStream::from(input),
            "Can't embed current file as it is not a directory",
        )
        .to_compile_error()
        .into();
    }

    let children = read_dir(&path, &path);
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

fn read_dir(base: &PathBuf, path: &PathBuf) -> Vec<TokenStream> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path).expect("Failed to list directory contents") {
        let entry = entry.expect("Failed to read entry");

        let path = entry.path();

        let path_str = path
            .strip_prefix(base)
            .expect("Failed to strip prefix of path")
            .to_str()
            .expect("Failed to get the string representation of PathBuf");

        let filetype = fs::metadata(&path)
            .expect("Failed to get file metadata")
            .file_type();

        if filetype.is_dir() {
            let children = read_dir(base, &path);
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
                    content: include_bytes!(#path_str),
                    path: ::std::path::PathBuf::from(#path_str),
                })
            });
        }
    }

    entries
}
