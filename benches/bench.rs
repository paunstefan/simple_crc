use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use simple_crc::*;

fn simplecrc8(c: &mut Criterion) {
    let bytes = vec![11u8; 1_000_000];

    let mut group = c.benchmark_group("crc8");
    group.throughput(Throughput::Bytes(1_000_000));
    group.bench_function("crc8", move |b| {
        b.iter(|| simple_crc8(&bytes, 0x07, 0x00, false, false, 0x00))
    });
    group.finish();
}

fn simplecrc16(c: &mut Criterion) {
    let bytes = vec![11u8; 1_000_000];

    let mut group = c.benchmark_group("crc16");
    group.throughput(Throughput::Bytes(1_000_000));
    group.bench_function("crc16", move |b| {
        b.iter(|| simple_crc16(&bytes, 0x1021, 0xffff, false, false, 0x0000))
    });
    group.finish();
}

fn simplecrc32(c: &mut Criterion) {
    let bytes = vec![11u8; 1_000_000];

    let mut group = c.benchmark_group("crc32");
    group.throughput(Throughput::Bytes(1_000_000));
    group.bench_function("crc32", move |b| {
        b.iter(|| simple_crc32(&bytes, 0x04C11DB7, 0xFFFFFFFF, false, false, 0xFFFFFFFF))
    });
    group.finish();
}

fn simplecrc64(c: &mut Criterion) {
    let bytes = vec![11u8; 1_000_000];

    let mut group = c.benchmark_group("crc64");
    group.throughput(Throughput::Bytes(1_000_000));
    group.bench_function("crc64", move |b| {
        b.iter(|| {
            simple_crc64(
                &bytes,
                0x42f0e1eba9ea3693,
                0xffffffffffffffff,
                false,
                false,
                0xffffffffffffffff,
            )
        })
    });
    group.finish();
}

criterion_group!(crc8_benches, simplecrc8);
criterion_group!(crc16_benches, simplecrc16);
criterion_group!(crc32_benches, simplecrc32);
criterion_group!(crc64_benches, simplecrc64);

criterion_main!(crc8_benches, crc16_benches, crc32_benches, crc64_benches);
