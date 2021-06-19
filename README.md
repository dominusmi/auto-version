# Automatically add `-v` and `--version` to your binaries

[![](https://img.shields.io/crates/v/auto-version.svg)](https://crates.io/crates/auto-version)
[![](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/auto-version/latest/auto_version/)

This crate contains an attribute macro, `auto_version`, which when applied to your
`main` function, will generate the output for `binary -v` or `binary --version`:
 ```shell
 $ ./binary -v 
 $ 0.1.0
 $ ./binary --version 
 $ 0.1.0
 ```


 ### Example
 ```rust
use auto_version::auto_version;

 #[auto_version]
 fn main() {
     // executed code
 }
 ```

⚠️ __Only works with cargo__ ⚠️

In the case where the code is not compiled with cargo, the version will be replaced
with the message
"\`auto_version\` macro only works for projects compiled with cargo".