//! Simple CRC (8, 16, 32, 64) library
//! # simple_crc
//!
//! simple_crc is an easy to use, no_std, lightweight CRC library for Rust.
//!
//! No lookup table used, so might be slower than other implementations, but with less memory used.
//!
//! ## Usage
//!
//! You have to pass the CRC function the byte slice you need the CRC calculated for, together with the parameters of the CRC, the polynomial, initial value, if the input or output needs to be reflected and output XOR value.
//!
//! ```rust
//! use simple_crc::*;
//!
//! let result = simple_crc8(b"123456789", 0x07, 0x00, false, false, 0x00);
//! assert_eq!(result, 0xF4);
//! ```
//!
//! The library supports CRCs with a length of 8, 16, 32 or 64 bits, each one uses a different function.
//!
#![no_std]
#![deny(missing_docs, trivial_casts, trivial_numeric_casts)]

/// Calculate CRC8
pub fn simple_crc8(
    data: &[u8],
    poly: u8,
    init: u8,
    refl_in: bool,
    refl_out: bool,
    xor_out: u8,
) -> u8 {
    calc_crc::<u8, 8>(data, poly, init, refl_in, refl_out, xor_out) as u8
}

/// Calculate CRC16
pub fn simple_crc16(
    data: &[u8],
    poly: u16,
    init: u16,
    refl_in: bool,
    refl_out: bool,
    xor_out: u16,
) -> u16 {
    calc_crc::<u16, 16>(data, poly, init, refl_in, refl_out, xor_out) as u16
}

/// Calculate CRC32
pub fn simple_crc32(
    data: &[u8],
    poly: u32,
    init: u32,
    refl_in: bool,
    refl_out: bool,
    xor_out: u32,
) -> u32 {
    calc_crc::<u32, 32>(data, poly, init, refl_in, refl_out, xor_out) as u32
}

/// Calculate CRC64
pub fn simple_crc64(
    data: &[u8],
    poly: u64,
    init: u64,
    refl_in: bool,
    refl_out: bool,
    xor_out: u64,
) -> u64 {
    calc_crc::<u64, 64>(data, poly, init, refl_in, refl_out, xor_out)
}

/// Compute generic CRC
fn calc_crc<T, const W: u8>(
    data: &[u8],
    poly: T,
    init: T,
    refl_in: bool,
    refl_out: bool,
    xor_out: T,
) -> u64
where
    T: Copy + Into<u64>,
{
    let width = W;
    let topbit = 1 << (width - 1);
    let mut remainder: u64 = init.into();

    for byte in data {
        let databyte = if refl_in { byte.reverse_bits() } else { *byte };
        // Bring the data byte into the remainder
        remainder ^= (databyte as u64) << (width - 8);

        for _ in 0..8 {
            // Check first bit, which is outside the polynomial
            if remainder & topbit != 0 {
                // Shift left to get rid of the topbit, as XORing will result in 0
                remainder = (remainder << 1) ^ poly.into();
            } else {
                remainder <<= 1;
            }
        }
    }

    remainder = if refl_out {
        // Need to shift it, because if it is smaller than 64 bits
        // the result will be in the most significant part, and we want
        // it on the other side so casting will work
        remainder.reverse_bits() >> (64 - width)
    } else {
        remainder
    };

    remainder ^ xor_out.into()
}
