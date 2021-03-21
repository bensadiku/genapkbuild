 Generate Android Makefile
=====

A command-line tool to automatically generate `Android.mk` for Android apk-s.


The genandroidmk tool will analyze apk for architectures, find NDK-generated libraries inside and generate a makefile for it.


This tool supports pre Kit-Kat style makefile generation too, by extracting NDK libraries from the APK and linking them into the makefile automatically. See [flags below for more information](#Flags). 

Ported from [go](https://github.com/northbright/genandroidmk) to rust with some more modifications, not backwards compatible with that tool.

## Binaries

If you have Rust compiler installed, simply clone and run `cargo build --release`. A binary `genandroidmk_rs` will generated in `target/release/`.

If you don't have the compiler installed, you can download the binary from the [release tab](https://github.com/bensadiku/genandroidmk_rs/releases)


## Flags

### Input flag [Required]
---

`-i --input <apk path>`

This flag is required, it supplies the path of the APK we want to generate the makefile for.

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
`-d --dex`

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