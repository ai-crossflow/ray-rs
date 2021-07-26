use std::env;
use std::path::PathBuf;

fn main() {
    let ray_src = PathBuf::from(env::var("RAY_SRC").unwrap());

    cxx_build::bridge("src/lib.rs")
        .include(ray_src.join("cpp/include").to_str().unwrap())
        .file("ray.cc")
        .flag_if_supported("-std=c++14")
        .compile("ray");

    println!("cargo:rerun-if-changed=src/lib.rs");
    // println!("cargo:rerun-if-changed=src/blobstore.cc");
    // println!("cargo:rerun-if-changed=include/blobstore.h");
}