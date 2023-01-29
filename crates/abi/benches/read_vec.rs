// use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
// use rkyv_wasm::{read_vec, read_vec_specialized};

// fn bench_fibs(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Fibonacci");
//     // group.sample_size(10);
//     for i in [50, 500, 80000].iter() {
//         let mut input = vec![0; i + 4];
//         input.as_mut_slice()[0..4].swap_with_slice(&mut i.to_le_bytes()[0..4]);

//         group.bench_with_input(
//             BenchmarkId::new("read_vec", i),
//             input.as_slice(),
//             |b, mut i| b.iter(|| read_vec::<u8>(&mut i)),
//         );
//         group.bench_with_input(
//             BenchmarkId::new("read_vec_specialized", i),
//             input.as_slice(),
//             |b, mut i| b.iter(|| read_vec_specialized::<u8>(&mut i)),
//         );
//     }
//     group.finish();
// }

// criterion_group!(benches, bench_fibs);
// criterion_main!(benches);
