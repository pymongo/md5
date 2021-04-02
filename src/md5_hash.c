// https://www.nayuki.io/res/fast-md5-hash-implementation-in-x86-assembly/md5-test.c
#include <stdint.h> // uint8_t, unint32_t
#include <stddef.h> // size_t
#include <string.h> // memset, memcpy

#define BLOCK_LEN 64  // In bytes
#define STATE_LEN 4  // In words

// multiple definition of `md5_compress'
extern void md5_compress(uint32_t state[STATE_LEN], const uint8_t block[BLOCK_LEN]);

void md5_hash(const uint8_t message[], size_t len, uint32_t hash[STATE_LEN]) {
	hash[0] = UINT32_C(0x67452301);
	hash[1] = UINT32_C(0xEFCDAB89);
	hash[2] = UINT32_C(0x98BADCFE);
	hash[3] = UINT32_C(0x10325476);
	
	#define LENGTH_SIZE 8  // In bytes
	
	size_t off;
	for (off = 0; len - off >= BLOCK_LEN; off += BLOCK_LEN)
		md5_compress(hash, &message[off]);
	
	uint8_t block[BLOCK_LEN] = {0};
	size_t rem = len - off;
	memcpy(block, &message[off], rem);
	
	block[rem] = 0x80;
	rem++;
	if (BLOCK_LEN - rem < LENGTH_SIZE) {
		md5_compress(hash, block);
		memset(block, 0, sizeof(block));
	}
	
	block[BLOCK_LEN - LENGTH_SIZE] = (uint8_t)((len & 0x1FU) << 3);
	len >>= 5;
	for (int i = 1; i < LENGTH_SIZE; i++, len >>= 8)
		block[BLOCK_LEN - LENGTH_SIZE + i] = (uint8_t)(len & 0xFFU);
	md5_compress(hash, block);
}
