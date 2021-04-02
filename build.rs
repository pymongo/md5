#[cfg(not(target_os = "linux"))]
compile_error!("libcrypto.so(openssl) only support on linux");

use std::process::Command;

fn main() {
    // openssl/md5.h
    println!("cargo:rustc-link-lib=dylib=crypto");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    // compile md5_compress.S and md5_hash.c in best Optimize level
    Command::new("gcc")
        .args(&[
            "-shared",
            "-O3",
            "src/md5_compress.S",
            "src/md5_hash.c",
            "-o",
        ])
        .arg(&format!("{}/md5_hash.o", out_dir))
        .status()
        .unwrap();

    // pack md5_hash.o to static linking library
    Command::new("ar")
        .current_dir(&std::path::Path::new(&out_dir))
        .args(&["crs", "libmd5_hash.a", "md5_hash.o"])
        .status()
        .unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=md5_hash");
    println!("cargo:rerun-if-changed=src/md5_hash.c");
}
