//! # Optimized Iterator
//!
//! `IntoOptimizedChunks` is an optimized iterator over `OptimizedChunks`of permutations.
//!
//! It is more efficient than `IntoChunks` because it store each computation,
//! in a fixed array that is stack allocated and efficiently copied from one job,
//! to the other
//!
//! Since each `OptimizedJob` must have a map with the frequency of each value,
//! to avoid using the heap allocated map, the original input in each `OptimizedJob`
//! is represented as an array where the index is the id of the original value,
//! and the frequency is the value stored in the index.
//!
//! Each `OptimizedChunks` has a map with the mapping between the index and the original value.
//! Since we have one `OptimizedChunks` for many permutations, the cost to allocate the map,
//! is negligible.
//!
//! This code can be further improved storing chunks of permutation in a fixed array,
// rather than on the heap allocated vector.
//!
//! `OptimizedChunks` is a sequence of permutations-
//! It is a `Display` to be written to output.
//! It is a `AsMut` to be updated with new permutations.
//!
//! `OptimizedJob` is the computational node to create a new permutation.
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

pub(crate) const PERMUTATION_FIXED_LENGTH: usize = 128;

type FixedArray = [usize; PERMUTATION_FIXED_LENGTH];

fn zeroed_fixed_array() -> FixedArray {
    [0; PERMUTATION_FIXED_LENGTH]
}

/// Optimized iterator over `OptimizedChunks`.
pub struct IntoOptimizedChunks<T> {
    job_queue: Vec<OptimizedJob>,
    size: usize,
    index_to_value: HashMap<usize, T>,
    permutation_size: usize,
}
// Initialize the iterator with the `job_queue` containing the root `OptimizedJob`.
/// The root `OptimizedJob` has the compressed form of the original input value..
impl<T: Copy + Eq + Hash> IntoOptimizedChunks<T> {
    pub(crate) fn new(values: Vec<T>, size: usize) -> Self {
        let permutation_size = values.len();
        let (compressed_values, index_to_value) = compress_values(values);

        Self {
            job_queue: vec![OptimizedJob::new(compressed_values)],
            size,
            index_to_value,
            permutation_size,
        }
    }
}

/// The iterator implementation to generate a single chunk of permutations.
/// It implements a breadth-first-search.
/// It terminates when the chunk is full
/// or there are no more permutations (the `job_queue` is empty).
impl<T: Copy> Iterator for IntoOptimizedChunks<T> {
    type Item = OptimizedChunk<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = OptimizedChunk::new(
            self.index_to_value.clone(),
            self.permutation_size,
            self.size,
        );

        while let Some(job) = self.job_queue.pop() {
            let next_jobs = job.compute_next_jobs();
            if let Some(first_job) = next_jobs.first() {
                if first_job.is_ready() {
                    chunk.as_mut().extend(
                        next_jobs
                            .into_iter()
                            .map(|completed_job| completed_job.permutation()),
                    );
                    if chunk.is_full() {
                        return Some(chunk);
                    }
                } else {
                    self.job_queue.extend(next_jobs)
                }
            }
        }
        if chunk.is_empty() {
            None
        } else {
            Some(chunk)
        }
    }
}

/// Compress the `values` into a fixed array: `A`, and generate a map: `H` to decode it.
/// The fixed array is such that at a given index: `i`:
/// `A[i]` is the frequency of `H[i]` in `values`, if `i` is a key present in `H`.
fn compress_values<T: Copy + Eq + Hash>(values: Vec<T>) -> (FixedArray, HashMap<usize, T>) {
    let mut value_to_index = HashMap::new();
    let mut i_th_distinct_value: usize = 0;
    let mut compressed_values = zeroed_fixed_array();
    let mut index_to_value = HashMap::new();
    for value in values.iter() {
        if let Some(idx) = value_to_index.get(value) {
            compressed_values[*idx] += 1;
        } else {
            value_to_index.insert(value, i_th_distinct_value);
            index_to_value.insert(i_th_distinct_value, *value);
            compressed_values[i_th_distinct_value] = 1;
            i_th_distinct_value += 1;
        }
    }
    (compressed_values, index_to_value)
}
/// Optimized chunks of compressed permutations.
pub struct OptimizedChunk<T> {
    /// the vector of compressed permutations
    permutations_compressed: Vec<FixedArray>,
    /// the map to decode compressed permutations
    index_to_value: HashMap<usize, T>,
    permutation_size: usize,
    size: usize,
}

impl<T> OptimizedChunk<T> {
    fn new(index_to_value: HashMap<usize, T>, permutation_size: usize, size: usize) -> Self {
        Self {
            permutations_compressed: vec![],
            index_to_value,
            permutation_size,
            size,
        }
    }
    fn is_full(&self) -> bool {
        self.permutations_compressed.len() == self.size
    }
    fn is_empty(&self) -> bool {
        self.permutations_compressed.is_empty()
    }
}

impl<T> AsMut<Vec<[usize; 128]>> for OptimizedChunk<T> {
    fn as_mut(&mut self) -> &mut Vec<[usize; 128]> {
        &mut self.permutations_compressed
    }
}
/// `Chunk` is a `Display` because it must be outputted.
/// This is where the `index_to_value` mapping to decode a compressed permutation is used.
impl<T: ToString> fmt::Display for OptimizedChunk<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.permutations_compressed
            .iter()
            .try_for_each(|permutation| {
                let last_permutation_index = self.permutation_size - 1;

                let permutation_without_last_value = permutation
                    .iter()
                    .take(last_permutation_index)
                    .fold(String::new(), |acc, index| {
                        acc + &self.index_to_value[index].to_string() + ","
                    });

                writeln!(
                    fmt,
                    "{}{}",
                    permutation_without_last_value,
                    &self.index_to_value[&permutation[(last_permutation_index)]].to_string()
                )
            })
    }
}

/// The computational unit.
#[derive(Copy, Clone)]
struct OptimizedJob {
    /// the remaining compressed values to use.
    compressed_values: FixedArray,
    /// the current compressed permutation
    compressed_permutation: FixedArray,
    /// this is the current permutation length.
    /// it is not the target permutation length.
    /// it is used to find the next index of `compressed_permutation`
    /// to add a new value
    permutation_length: usize,
}

impl OptimizedJob {
    /// Initialize a new `OptimizedJob`.
    fn new(compressed_values: FixedArray) -> Self {
        Self {
            compressed_values,
            compressed_permutation: zeroed_fixed_array(),
            permutation_length: 0,
        }
    }

    /// Given a parent `OptimizedJob`, it is possible to generate new jobs,
    /// with one more value in `compressed_permutation`
    /// and a decreased frequency in `compressed_values`.
    fn compute_next_jobs(self) -> Vec<OptimizedJob> {
        let mut result = vec![];

        for (idx, freq) in self.compressed_values.iter().enumerate() {
            if *freq > 0 {
                result.push(self.with_new_value(&idx))
            }
        }
        result
    }

    /// Create a new `OptimizedJob` given a new `value` to add inside the `compressed_permutation`,
    /// at index: `permutation_length`.
    /// The frequency of the `value` must be decreased in the new `OptimizedJob` instance.
    fn with_new_value(&self, value: &usize) -> Self {
        let mut frequencies = self.compressed_values;
        frequencies[*value] -= 1;

        let mut new_permutation = self.compressed_permutation;
        new_permutation[self.permutation_length] = *value;

        let mut new_job = Self::new(frequencies);
        new_job.compressed_permutation = new_permutation;
        new_job.permutation_length = self.permutation_length + 1;
        new_job
    }
    /// Check if the `OptimizedJob` has found a permutation,
    /// and consequently it cannot generate any children jobs.
    /// This happens when the frequency of each value is zero,
    /// and consequently `compressed_values` has all only zeros.
    fn is_ready(&self) -> bool {
        self.compressed_values.eq(&zeroed_fixed_array())
    }

    /// Get the permutation generated by the `OptimizedJob`.
    /// It is a valid permutation of correct length
    /// only if `is_ready()` is true.
    fn permutation(self) -> FixedArray {
        self.compressed_permutation
    }
}
