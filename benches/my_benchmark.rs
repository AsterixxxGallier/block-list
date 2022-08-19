use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
// use criterion_cycles_per_byte::CyclesPerByte;
use itertools::iterate;
use block_list::BlockList;

fn push_ones(n: u64) -> usize {
    let mut list = BlockList::new();
    for _ in 0..n {
        list.push(1);
    }
    list.len()
}

fn criterion_benchmark(c: &mut Criterion) {
    // let mut group = c.benchmark_group("push_ones");
    // for n in iterate(1, |n| 10 * n).take(8) {
    //     group.throughput(Throughput::Elements(n));
    //     group.bench_with_input(BenchmarkId::from_parameter(n), &n,
    //                            |b, n| b.iter(||
    //                                push_ones(black_box(*n))));
    // }
    // group.finish();
    let mut list = BlockList::new();
    c.bench_function("push_ones", |b| b.iter(||
                                                 list.push(black_box(23))));
    println!("{:?}", list.len());
}

criterion_group!(
    name = benches;
    // config = Criterion::default().with_measurement(CyclesPerByte);
    config = Criterion::default();
    targets = criterion_benchmark
);
criterion_main!(benches);