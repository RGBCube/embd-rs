# embed-rs

A super simple file and directory embedding crate,
that loads files from the filesystem in debug mode,
allowing for quick edit-and-test cycles without compilation.

It is also super efficient, and does not heap allocate when the
files are embedded on release mode by utilizing `std::borrow::Cow`.

On release mode it falls back to `include_str!`, `include_bytes!`
and our own custom `include_dir!`-like implementation.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
embed = { git = "https://github.com/RGBCube/embed-rs" }

[patch.crates-io]
proc-macro2 = { git = "https://github.com/RGBCube/proc-macro2" }
```

Then you can use this crate as so:

```rs
let contents: Cow<'_, str> = embed::string!("path/to/file.txt");
let bytes: Cow<'_, [u8]> = embed::bytes!("path/to/image.png");

let dir: embed::Dir = embed::dir!("path/to");
let files: Vec<embed::File> = dir.flatten();
```

I am not sure what name to publish this
crate in, lmk if you find a good one.

## License

```
MIT License

Copyright (c) 2023-present RGBCube

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
