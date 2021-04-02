/*!
# Rust 生态各种 md5 库的性能比较

## 其它编程语言的md5 API
```c
#include <openssl/md5.h> // MD5 # gcc -lcrypto
#include <stdio.h>       // sprintf, printf
#include <string.h>      // strlen

void md5_hash(const char *input_str, char md5_hash_ouput[32]) {
    unsigned char md5_output[16];
    MD5((unsigned char *)input_str, strlen(input_str), md5_output);
    for (int i=0; i<16; i++) {
        sprintf(md5_hash_ouput+2*i, "%02x", md5_output[i]);
    }
}

int main() {
    const char* md5_hash_input = "The quick brown fox jumps over the lazy dog";
    char md5_hash_output[32]; // TODO add '\0' to end of str?
    md5_hash(md5_hash_input, md5_hash_output);
    printf("md5_hash_input =%s\nmd5_hash_output=%s\n", md5_hash_input, md5_hash_output);
    return 0;
}
```

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

/// #include <openssh/md5.h>
/// openssl在linux系统上一般都是预装的库，像actix-web之类涉及网络传输的很多库都依赖到openssl
pub fn openssl_md5(input: &[u8]) -> String {
    #[link(name = "crypto", kind = "dylib")]
    extern "C" {
        /// unsigned char *MD5(const unsigned char *d, unsigned long n, unsigned char *md);
        fn MD5(input: *const u8, input_len: usize, output: &mut [u8; 16]) -> *mut u8;
    }
    let mut output = [0u8; 16];
    unsafe {
        MD5(input.as_ptr() as *const u8, input.len(), &mut output);
    }
    let output = u128::from_be_bytes(output); // transmute 用的是 native_endian，最好还是显式的调用 from_be_bytes
    format!("{:x}", output)
}

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

/**
md5-asm库的汇编源码来自于nayuki.io这个网站
https://www.nayuki.io/page/fast-md5-hash-implementation-in-x86-assembly
而且Rust-Crypto组织也就简单的给汇编的`md5_compress`方法包一层，事实上这个函数还不能用
要结合另一个`md5_hash`的C函数一起使用

## `.S` 和 `.s` 后缀的汇编源文件的区别
大写S后缀结尾的汇编源文件可以有类似C语言的`#define`这样的预处理指令
gcc编译*.S的汇编源文件时会先进行宏展开(预处理)，而*.s的源文件则不会预处理
所以nayuki这个汇编源码如果命名成`*.s`的后缀名，则`gcc -c md5_compress.s`编译时会报错
```
[w@w-manjaro src]$ as md5_compress.s
md5_compress.s: Assembler messages:
md5_compress.s:49: Error: bad register name `%c'
md5_compress.s:49: Error: junk at end of line, first unrecognized character is `\'
md5_compress.s:50: Error: bad register name `%a'
```
而且`*.S`的源文件不能直接用as去编译，因为as默认不进行宏的预处理，要用`gcc -c`去编
*/
pub fn md5_asm(input: &[u8]) -> String {
    #[link(name = "md5_hash", kind = "static")]
    extern "C" {
        /// void md5_hash(const uint8_t message[], size_t len, uint32_t hash[4]);
        fn md5_hash(input: *const u8, len: usize, output: &mut [u32; 4]);
    }
    let mut output = [0u32; 4];
    unsafe {
        md5_hash(input.as_ptr() as *const u8, input.len(), &mut output);
    }
    let output = unsafe {
        // convert native-endian to bigger-endian after transmute
        std::mem::transmute::<[u32; 4], u128>(output).to_be()
    };
    format!("{:x}", output)
}

#[test]
fn test_md5() {
    for &(input, output) in MD5_TEST_CASES.iter() {
        assert_eq!(openssl_md5(input), output);
        assert_eq!(md_5_crate_md5(input), output);
        assert_eq!(md5_crate_md5(input), output);
        assert_eq!(md5_asm(input), output);
    }
}
