use criterion::{criterion_group, criterion_main, Criterion};
use picocolors::{bg_black, formatter as formatter_inline};

fn main_bench(b: &mut Criterion) {
  let mut group = b.benchmark_group("benchmark");
  group.sample_size(1000);
  group.bench_function("picocolor_rs", |b| {
    b.iter(|| formatter_inline("111", "333")("222"))
  });
  group.bench_function("picocolor_rs bg_black", |b| b.iter(|| bg_black("222")));
}

criterion_group!(benches, main_bench);
criterion_main!(benches);
