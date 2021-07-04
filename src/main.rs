use std::cmp::max;
use std::convert::TryInto;
use std::io::{self, BufRead, Write};

use perm::{IntoChunks, IntoOptimizedChunks, Permutations};

const OPTIMAL_THREADS_NUMBER: usize = 256;

fn main() {
    let reader = io::stdin();

    let text = reader
        .lock()
        .lines()
        .next()
        .expect("Empty input")
        .expect("Error reading input");
    let input = text.as_str();

    let permutations: Permutations<&str> = input.try_into().expect("Error reading input text");

    let chunk_size = max(
        16,
        factorial(permutations.length()) / OPTIMAL_THREADS_NUMBER,
    );
    if permutations.can_be_optimized() {
        eprintln!("Using optimized iterator");
        generate_optimized_permutations(permutations.into_optimized_chunks(chunk_size))
    } else {
        eprintln!("Using normal iterator");
        generate_permutations(permutations.into_chunks(chunk_size))
    }
}

fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        factorial(n - 1) * n
    }
}

fn generate_optimized_permutations(iterator: IntoOptimizedChunks<&str>) {
    crossbeam::scope(|scope| {
        let handles = iterator
            .map(|chunk| {
                scope.spawn(move |_| {
                    io::stdout()
                        .write_all(chunk.to_string().as_ref())
                        .expect("Error writing data")
                })
            })
            .collect::<Vec<_>>();

        handles.into_iter().for_each(|v| {
            v.join()
                .expect("Error waiting optimized_permutations to terminate");
        })
    })
    .expect("Error generating optimized permutations")
}

fn generate_permutations(iterator: IntoChunks<&str>) {
    crossbeam::scope(|scope| {
        let handles = iterator
            .map(|chunk| {
                scope.spawn(move |_| {
                    io::stdout()
                        .write_all(chunk.to_string().as_ref())
                        .expect("Error writing data")
                })
            })
            .collect::<Vec<_>>();
        handles.into_iter().for_each(|v| {
            v.join()
                .expect("Error waiting generate_permutations to terminate");
        })
    })
    .expect("Error generating permutations")
}
