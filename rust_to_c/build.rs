fn main() {
    let mut build = cc::Build::new();
    build.file("external/test.c");
    build.include("external");
    build.out_dir("external/out");
    build.compile("test");

    println!("cargo:rustc-link-search=rust_to_c/external/out");
    println!("cargo:rustc-link-lib=test");

    println!("cargo::rerun-if-changed=rust_to_c/external/test.c");

    let bindings = bindgen::Builder::default()
        .header("external/test.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
