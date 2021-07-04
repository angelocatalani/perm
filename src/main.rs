//! # The `perm` entry point.
//!
//! Read the input from `stdin` and generate the permutations to `stdout`.
//!
//! A valid input is expected to contain only comma separated numbers.
//! The permutations are computed sequentially in chunks of a given size,
//! and written to `stdout` in a new thread.
//! In this way the blocking I/O operations do not block the computations of the next chunk.
//!
//! The chunk size is such that to have `OPTIMAL_THREADS_NUMBER` total threads.
//! This value has been found empirically after some benchmarks on my pc.
//!
//! If the input text is short enough (`PERMUTATION_FIXED_LENGTH=128` elements) is is possible to use an optimized version of the algorithm.
//! otherwise it fallbacks to the slower version.
//!
//! # Panic
//!
//! If the input is empty or does not contain comma separated numbers

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
    eprintln!("Done")
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

        handles.into_iter().for_each(|h| {
            h.join()
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
        handles.into_iter().for_each(|h| {
            h.join()
                .expect("Error waiting generate_permutations to terminate");
        })
    })
    .expect("Error generating permutations")
}
