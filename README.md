 Generate AOSP build system files from APK
=====

⚠️ This library is a Work in Progress! ⚠️

A command-line tool to automatically generate `Android.mk` or `Android.bp` from Android APK-s.

The genapkbuild tool will analyze APK for architectures, find NDK-generated libraries inside and generate makefile (`Android.mk`) or soong (`Android.bp`) for it. By default it will generate make files but it can be changed to generate soong files (Android.bp) by passing different flags.


This tool supports pre Kit-Kat style makefile generation too, by extracting .so libraries from the APK and linking them into the makefile automatically. 

See [flags below for more information](#Flags). 

Inspired from northbright's tool written [Go](https://github.com/northbright/genandroidmk), ported to Rust and built on top of it, not backwards compatible with that tool (for the most part).

## Binaries

If you have Cargo installed, you can install this tool by running `cargo install genapkbuild`
To build from source, you need Rust compiler, clone this repo and run `cargo build --release`. A binary `genapkbuild` will generated in `target/release/`.

If you don't have the compiler installed, you can download the binary from the [release tab](https://github.com/bensadiku/genapkbuild/releases)


## Flags

### Input flag [Required]
---

`-i --input <apk path>`

This flag is required, it supplies the path of the APK we want to generate the makefile or blueprint for.

### Soong (Android.bp) flag
---

`-s --soong`

This flag will create `Android.bp` instead of `Android.mk`

### Extract flag
---

`-e --extract`

This flag will create pre Android 5.0 style makefiles by extracting/copying native libraries outside of APK into `/lib/<abi>/lib<name>.so` and linking them to the makefile. Disabled by default.

### Architecture (ABI) flag
---

`-a --arch <ABI>`

This flag will allow you to set a default architecture for the APK you are bundling in BSP.
This is neccessaary when more than one architecture is found and the tool doesn't know which one to prefer.
If there's only one architecture supported, it will auto-pick that.


### Privileged flag
---

`-p --privileged`

This flag will make the apk a privileged system app located in priv-app.


### Dex Pre opt
---
`-d --dex <true or false>`

Pre-optimization can also be enabled or disabled on an individual app basis by specifying this flag.

### Name flag
---

`-n --name <APK name>`

If the apk name is different from the one supplied with the `-i --input` flag, you may change that with this flag. If not supplied, it will get whatever was passed in the input flag. 

### Verbose flag
---

`-v --verbose <APK name>`

Enable verbose debug logging, by default disabled.

### Version flag
---

`-V --version`

Prints version information.

### Help flag
---

`-h --help`

Prints usage information about the tool.


License
---
Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.