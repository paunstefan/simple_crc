# simple_crc

simple_crc is an easy to use, no_std, lightweight CRC library for Rust.

No lookup table used, so might be slower than other implementations, but with less memory used.

## Usage

You have to pass the CRC function the byte slice you need the CRC calculated for, together with the parameters of the CRC, the polynomial, initial value, if the input or output needs to be reflected and output XOR value.

```rust
use simple_crc::*;

let result = simple_crc8(b"123456789", 0x07, 0x00, false, false, 0x00);
assert_eq!(result, 0xF4);
```

The library supports CRCs with a length of 8, 16, 32 or 64 bits, each one uses a different function.