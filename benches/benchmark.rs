use criterion::{criterion_group, criterion_main, Criterion};
use perm::{ Permutations};
use std::thread;
use std::thread::JoinHandle;


fn generate_string_new_thread<T: 'static + ToString+Send+Sync>(chunk: T) -> JoinHandle<String> {
    let handle = thread::spawn(move || {
        chunk.to_string()
    });
    handle

}

fn permutations_into_chunks(c: &mut Criterion) {
    c.bench_function("Permutation IntoChucks", |b| {
        b.iter(|| {
            // linter warning forces the sequential execution
            let handles = Permutations::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
                .into_chunks(1000000)
                .map(generate_string_new_thread)
                .collect::<Vec<JoinHandle<String>>>();
            handles.into_iter().map(|h|h.join()).for_each(drop);

        })
    });
}

fn permutations_into_optimized_chunks(c: &mut Criterion) {
    c.bench_function("Permutation IntoOptimizedChucks", |b| {
        b.iter(|| {
            // linter warning forces the sequential execution
            let handles= Permutations::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
                .into_optimized_chunks(1000000).unwrap()
                .map(generate_string_new_thread)
                .collect::<Vec<JoinHandle<String>>>();
            handles.into_iter().map(|h|h.join()).for_each(drop);
        })

    });
}


criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = permutations_into_chunks, permutations_into_optimized_chunks
}

criterion_main!(benches);
