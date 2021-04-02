#![feature(test)]
extern crate md5_benchmark;
extern crate test;

use md5_benchmark::{md5_asm, md5_crate_md5, md_5_crate_md5, openssl_md5, MD5_TEST_CASES};

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

#[bench]
fn bench_md5_asm(b: &mut test::Bencher) {
    b.iter(|| {
        for &(input, output) in MD5_TEST_CASES.iter() {
            assert_eq!(md5_asm(input), output);
        }
    });
}
