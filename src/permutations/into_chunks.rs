//! # Iterator
//!
//! `IntoChunks` is an iterator over `Chunk`of permutations.
//!
//! `Chunk` is a sequence of permutations-
//! It is a `Display` to be written to output.
//! It is a `AsMut` to be updated with new permutations.
//!
//! `Job` is the computational node to create a new permutation.
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

/// Iterator over `Chunks`
pub struct IntoChunks<T> {
    job_queue: Vec<Job<T>>,
    size: usize,
}

/// Initialize the iterator with the `job_queue` containing the root `Job`.
/// The root `Job` has the hash map to associate the frequency to each permutation input value.
impl<T: Copy + Eq + Hash> IntoChunks<T> {
    pub(crate) fn new(values: Vec<T>, size: usize) -> Self {
        let permutation_length = values.len();
        Self {
            job_queue: vec![Job::new(values_with_frequency(values), permutation_length)],
            size,
        }
    }
}

/// The iterator implementation to generate a single chunk of permutations.
/// It terminates when the chunk is full
/// or there are no more permutations (the `job_queue` is empty).
impl<T: Copy + Eq + Hash> Iterator for IntoChunks<T> {
    type Item = Chunk<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Chunk::new(self.size);

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

/// Compute the hashmap with the frequency for each value.
fn values_with_frequency<T: Eq + Hash>(values: Vec<T>) -> HashMap<T, usize> {
    let mut values_with_frequency = HashMap::new();
    for value in values {
        *values_with_frequency.entry(value).or_insert(0) += 1;
    }
    values_with_frequency
}

/// Chunk of permutations.
pub struct Chunk<T> {
    permutations: Vec<Vec<T>>,
    size: usize,
}

impl<T> Chunk<T> {
    fn new(size: usize) -> Self {
        Self {
            permutations: vec![],
            size,
        }
    }
    fn is_full(&self) -> bool {
        self.permutations.len() == self.size
    }
    fn is_empty(&self) -> bool {
        self.permutations.is_empty()
    }
}

impl<T> AsMut<Vec<Vec<T>>> for Chunk<T> {
    fn as_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.permutations
    }
}

/// `Chunk` is a `Display` because it must be outputted.
impl<T: ToString> fmt::Display for Chunk<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.permutations.iter().try_for_each(|permutation| {
            let last_permutation_index = permutation.len() - 1;

            let permutation_without_last_value = permutation
                .iter()
                .take(last_permutation_index)
                .fold(String::new(), |acc, value| acc + &value.to_string() + ",");

            writeln!(
                fmt,
                "{}{}",
                permutation_without_last_value,
                &permutation[last_permutation_index].to_string()
            )
        })
    }
}

/// The computational unit.
struct Job<T> {
    /// the remaining values to use, with their frequency.
    /// the hashmap allows to ignore the duplicated permutations.
    values_with_positive_frequency: HashMap<T, usize>,

    /// the current generate permutation.
    permutation: Vec<T>,

    /// the target permutation length.
    /// this is the same for all jobs.
    permutation_length: usize,
}

impl<T: Copy + Eq + Hash> Job<T> {
    /// Initialize a new `Job` ignoring the values with zero frequency.
    fn new(values_with_frequency: HashMap<T, usize>, permutation_length: usize) -> Self {
        Self {
            values_with_positive_frequency: values_with_frequency
                .into_iter()
                .filter(|(_, frequency)| *frequency > 0)
                .collect(),
            permutation: vec![],
            permutation_length,
        }
    }

    /// Given a parent `Job`, it is possible to generate new jobs,
    /// with one more value in `permutation`
    /// and a decreased frequency in `values_with_positive_frequency`.
    fn compute_next_jobs(self) -> Vec<Job<T>> {
        let mut next_jobs = vec![];
        for (value, _) in self.values_with_positive_frequency.iter() {
            let next_job = self.with_new_value(&value);
            next_jobs.push(next_job);
        }
        next_jobs
    }

    /// Create a new `Job` given a new `value` to add inside the `permutation`.
    /// The frequency of the `value` must be decreased in the new `Job` instance
    /// and eventually deleted if the frequency become zero.
    fn with_new_value(&self, value: &T) -> Self {
        let mut new_values_with_frequency = self.values_with_positive_frequency.clone();
        decrease_or_remove_positive_frequency(&mut new_values_with_frequency, value);

        let mut new_permutation = self.permutation.clone();
        new_permutation.push(*value);
        Self {
            values_with_positive_frequency: new_values_with_frequency,
            permutation: new_permutation,
            permutation_length: self.permutation_length,
        }
    }

    /// Get the permutation generated by the `Job`.
    /// It is a valid permutation of correct length
    /// only if it has the same length of `permutation_length`.
    fn permutation(self) -> Vec<T> {
        self.permutation
    }

    /// Check if the `Job` has found a permutation,
    /// and consequently it cannot generate any children jobs.
    /// It is a valid permutation of correct length
    /// only if `is_ready()` is true.
    fn is_ready(&self) -> bool {
        self.permutation.len() == self.permutation_length
    }
}

/// Decrease the frequency of `value` from `values_with_frequency`,
/// and it deletes the new entry if the resulting frequency is zero.
fn decrease_or_remove_positive_frequency<T: Copy + Hash + Eq>(
    values_with_frequency: &mut HashMap<T, usize>,
    value: &T,
) {
    match values_with_frequency.entry(*value) {
        Entry::Occupied(mut frequency) => {
            if *frequency.get() == 1 {
                frequency.remove_entry();
            } else {
                *frequency.get_mut() -= 1
            }
        }
        Entry::Vacant(_) => {}
    }
}
