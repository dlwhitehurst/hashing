fn main() {


// initial hashes a, b, c, d, e, f, g, h

    let _a:u32 = 0x6A09E667;
    let _b:u32 = 0xBB67AE85;
    let _c:u32 = 0x3C6EF372;
    let _d:u32 = 0xA54FF53A;
    let _e:u32 = 0x510E527F;
    let _f:u32 = 0x9B05688C;
    let _g:u32 = 0x1F83D9AB;
    let _h:u32 = 0x5BE0CD19;

// array of 64 constant words, or the first 32 bits of the fractional parts of the cube roots
// of the first 64 primes

    let _k: [u32; 64] =[ 0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2 ];

    println!("Testing an array item (63) = 0x{:x}", _k[62]);
    println!("0x{:x}", efg_compress(0x19a4c116, 0x4a7484aa, 0xbef9a3f7));
    println!("0x{:x}", abc_compress(0x19a4c116, 0x4a7484aa, 0xbef9a3f7));

}

// bitwise and function
// x & y;

// bitwise or function
// x | y;

// bitwise xor function
// x ^ y;


// e-f-g hash compression
fn efg_compress(e: u32, f: u32, g: u32) -> u32 {
    let y = !e;
    (e & f) ^ (y & g)
}

// a-b-c hash compression
fn abc_compress(a: u32, b: u32, c: u32) -> u32 {
    (a & c) ^ (a & b) ^ (b & c)
}

