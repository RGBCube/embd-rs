# embd-rs

A super simple file and directory embedding crate,
that loads files from the filesystem on debug mode,
allowing for quick edit-and-test cycles without compilation.

It is also super efficient, and does not heap allocate when the
files are embedded on release mode by utilizing `std::borrow::Cow`.

On release mode it falls back to `include_str!`, `include_bytes!`
and our own custom `include_dir!`-like implementation.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
embd = "0.1"
```

Then you can use this crate like so:

```rs
let contents: Cow<'_, str> = embd::string!("path/to/file.txt");
let bytes: Cow<'_, [u8]> = embd::bytes!("path/to/image.png");

let dir: embd::Dir = embd::dir!("path/to");
let files: Vec<embd::File> = dir.flatten();
```

Note that you will need to enable the `procmacro2_semver_exempt` cfg
option to use this crate, you can enable it like so, by putting this in
`.cargo/config.toml` in the project root:

```toml
[build]
rustflags = [ "--cfg", "procmacro2_semver_exempt" ]
```

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
