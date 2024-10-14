use std::env;

#[rustfmt::skip]
const HEADER: &str = 
r#"/* THIS FILE IS GENERATED, DO NOT EDIT!*/

#pragma once"#;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo::rerun-if-changed=c_to_rust/src/lib.rs");
    println!("cargo:rustc-link-lib=ntdll");

    cbindgen::Builder::new()
        .with_header(HEADER)
        .with_tab_width(4)
        .with_no_includes()
        .with_cpp_compat(true)
        .with_crate(crate_dir)
        .with_sys_include("cstdint")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/bindings.h");
}
