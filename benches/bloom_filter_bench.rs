use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::RngCore;
use stellarconduit_core::gossip::bloom::SlidingBloomFilter;

fn bench_sliding_bloom_filter(c: &mut Criterion) {
    let mut filter = SlidingBloomFilter::new(10000, 0.01);
    let mut rnd = rand::thread_rng();

    c.bench_function("check_and_add", |b| {
        b.iter(|| {
            let mut msg_id = [0u8; 32];
            rnd.fill_bytes(&mut msg_id);
            black_box(filter.check_and_add(black_box(&msg_id)));
        });
    });
}

criterion_group!(benches, bench_sliding_bloom_filter);
criterion_main!(benches);
