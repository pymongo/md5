#![feature(test)]
extern crate test;
extern crate md5_benchmark;

use md5_benchmark::{MD5_TEST_CASES, openssl_md5, md_5_crate_md5, md5_crate_md5};

#[bench]
fn bench_openssl_md5(b: &mut test::Bencher) {
    b.iter(|| {
        for &(input, output) in MD5_TEST_CASES.iter() {
            assert_eq!(openssl_md5(input), output);
        }
    });
}

#[bench]
fn bench_md_5_crate_md5(b: &mut test::Bencher) {
    b.iter(|| {
        for &(input, output) in MD5_TEST_CASES.iter() {
            assert_eq!(md_5_crate_md5(input), output);
        }
    });
}

#[bench]
fn bench_md5_crate_md5(b: &mut test::Bencher) {
    b.iter(|| {
        for &(input, output) in MD5_TEST_CASES.iter() {
            assert_eq!(md5_crate_md5(input), output);
        }
    });
}
