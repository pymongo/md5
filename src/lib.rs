//! Rust 生态各种 md5 库的性能比较

/// #include <openssh/md5.h>
pub fn openssl_md5(input: &[u8]) -> String {
    #[link(name = "crypto", kind = "dylib")]
    extern "C" {
        fn MD5(input: *const u8, input_len: usize, output: &mut [u8; 16]);
    }
    let mut output = [0u8; 16];
    unsafe {
        MD5(input.as_ptr() as *const u8, input.len(), &mut output);
    }
    let output = u128::from_be_bytes(output); // transmute 用的是 native_endian，最好还是显式的调用 from_be_bytes
    format!("{:x}", output)
}

/**
```python
import hashlib
hasher = hashlib.new('md5')
hasher.update("apple".encode())  # or b"apple"
hasher.hexdigest()
```

```ruby
require 'digest'
md5 = Digest::MD5.new
md5.update "apple"  # or md5 << "apple"
md5.hexdigest
```
*/
pub fn md_5_crate_md5(input: &[u8]) -> String {
    use md_5::{Digest, Md5};
    let mut hasher = Md5::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

pub fn md5_crate_md5(input: &[u8]) -> String {
    format!("{:x}", md5::compute(input))
}

/// test cases from wikipidiea: https://en.wikipedia.org/wiki/MD5#MD5_hashes
pub const MD5_TEST_CASES: [(&[u8], &str); 3] = [
    (
        b"The quick brown fox jumps over the lazy dog",
        "9e107d9d372bb6826bd81d3542a419d6",
    ),
    (
        b"The quick brown fox jumps over the lazy dog.",
        "e4d909c290d0fb1ca068ffaddf22cbd0",
    ),
    (b"", "d41d8cd98f00b204e9800998ecf8427e"),
];

#[test]
fn test_md5() {
    for &(input, output) in MD5_TEST_CASES.iter() {
        assert_eq!(openssl_md5(input), output);
        assert_eq!(md_5_crate_md5(input), output);
        assert_eq!(md5_crate_md5(input), output);
    }
}
