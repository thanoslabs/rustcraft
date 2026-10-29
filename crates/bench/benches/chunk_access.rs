use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn chunk_access(c: &mut Criterion) {
    let blocks = [0_u16; 16 * 16 * 16];
    c.bench_function("dense_chunk_read", |b| {
        b.iter(|| {
            let index = black_box(7 * 256 + 5 * 16 + 3);
            black_box(blocks[index])
        });
    });
}

criterion_group!(benches, chunk_access);
criterion_main!(benches);
