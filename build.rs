#[cfg(not(target_os = "linux"))]
compile_error!("libcrypto.so(openssl) only support on linux");

fn main() {
    // openssl/md5.h
    println!("cargo:rustc-link-lib=dylib=crypto");
}
