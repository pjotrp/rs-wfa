use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-search=native=libwfa");
    // println!("cargo:rustc-link-lib=static=wfa");
    println!("cargo:rustc-link-lib=wfa");

    let bindings = bindgen::Builder::default()
        .clang_arg("-IWFA")
        // .clang_arg("-includeWFA/")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_NORMAL")
        // .blacklist_type("FP_NAN")
        // .blacklist_type("FP_INFINITE")
        // .blacklist_type("FP_ZERO")
        // .blacklist_type("FP_SUBNORMAL")
        // .blacklist_type("FP_NORMAL")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
