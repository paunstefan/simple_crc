#[cfg(test)]
mod tests {
    use simple_crc::*;
    #[test]
    fn test_crc8() {
        let result = simple_crc8(b"123456789", 0x07, 0x00, false, false, 0x00);
        assert_eq!(result, 0xf4);

        let result = simple_crc8(b"123456789", 0x9b, 0xff, false, false, 0x00);
        assert_eq!(result, 0xda);

        let result = simple_crc8(b"123456789", 0x1d, 0xff, true, true, 0x00);
        assert_eq!(result, 0x97);
    }

    #[test]
    fn test_crc16() {
        let result = simple_crc16(b"123456789", 0x1021, 0xffff, false, false, 0x0000);
        assert_eq!(result, 0x29b1);

        let result = simple_crc16(b"123456789", 0x8005, 0x0000, true, true, 0x0000);
        assert_eq!(result, 0xbb3d);
    }

    #[test]
    fn test_crc32() {
        let result = simple_crc32(
            b"123456789",
            0x04C11DB7,
            0xFFFFFFFF,
            false,
            false,
            0xFFFFFFFF,
        );
        assert_eq!(result, 0xFC891918);

        let result = simple_crc32(b"123456789", 0x04C11DB7, 0xFFFFFFFF, true, true, 0xFFFFFFFF);
        assert_eq!(result, 0xCBF43926);
    }

    #[test]
    fn test_crc64() {
        let result = simple_crc64(
            b"123456789",
            0x42f0e1eba9ea3693,
            0xffffffffffffffff,
            false,
            false,
            0xffffffffffffffff,
        );
        assert_eq!(result, 0x62ec59e3f1a4f00a);

        let result = simple_crc64(
            b"123456789",
            0x42f0e1eba9ea3693,
            0x0000000000000000,
            false,
            false,
            0x0000000000000000,
        );
        assert_eq!(result, 0x6c40df5f0b497347);

        let result = simple_crc64(
            b"123456789",
            0x42f0e1eba9ea3693,
            0xffffffffffffffff,
            true,
            true,
            0xffffffffffffffff,
        );
        assert_eq!(result, 0x995dc9bbdf1939fa);
    }
}
