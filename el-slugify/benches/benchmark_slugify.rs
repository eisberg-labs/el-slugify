use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use el_slugify::{slugify as my_slugify_func};


fn criterion_benchmark(c: &mut Criterion) {
    let binding = std::iter::repeat("#% MaČKA mački grize rep! (RIB-a) ~*").take(10000).collect::<String>();
    let long_string = black_box(binding.as_str());

    let mut group = c.benchmark_group("Slugify");

    group.bench_with_input(BenchmarkId::new("el_slugify", 1), long_string, |bencher, input| {
        bencher.iter(|| my_slugify_func(input));
    });

    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
