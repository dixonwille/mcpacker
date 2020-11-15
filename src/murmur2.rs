use byteorder::{ByteOrder, LittleEndian};

// Ported from
// https://sites.google.com/site/murmurhash/ using MurmurHash2.cpp

const M: u32 = 0x5bd1e995;
const R: u32 = 24;

pub fn murmurhash2_32(key: &[u8], seed: u32) -> u32 {
    // Initialize the hash to a 'random' value
    // Can't write an incremental version as we need to know length ahead of time
    // Murmur2A solves this but is not what this crate uses
    let mut h = seed ^ (key.len() as u32);

    let mut chunks = key.chunks_exact(4);
    while let Some(chunk) = chunks.next() {
        // Make sure we are using LittleEndian
        h = mix(h, LittleEndian::read_u32(chunk));
    }

    // Handle the last few bytes of the input array
    h = tail(h, chunks.remainder());

    // Do a few final mixes of the hash to ensure the last few
    // bytes are well-incorporated.
    h ^= h >> 13;
    h = h.wrapping_mul(M);
    h ^ (h >> 15)
}

// returns value of h
fn mix(mut h: u32, mut k: u32) -> u32 {
    k = k.wrapping_mul(M);
    k ^= k >> R;
    k = k.wrapping_mul(M);

    h = h.wrapping_mul(M);
    h ^ k
}

fn tail(mut h: u32, remainder: &[u8]) -> u32 {
    match remainder.len() {
        3 => {
            h ^= u32::from(remainder[2]) << 16;
            h ^= u32::from(remainder[1]) << 8;
            h ^= u32::from(remainder[0]);
            h.wrapping_mul(M)
        }
        2 => {
            h ^= u32::from(remainder[1]) << 8;
            h ^= u32::from(remainder[0]);
            h.wrapping_mul(M)
        }
        1 => {
            h ^= u32::from(remainder[0]);
            h.wrapping_mul(M)
        }
        _ => h,
    }
}
