use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

pub(crate) const PERMUTATION_FIXED_LENGTH: usize = 128;

type FixedArray = [usize; PERMUTATION_FIXED_LENGTH];

fn zeroed_fixed_array() -> FixedArray {
    [0; PERMUTATION_FIXED_LENGTH]
}

pub struct IntoOptimizedChunks<T> {
    job_queue: Vec<OptimizedJob>,
    size: usize,
    index_to_value: HashMap<usize, T>,
    permutation_size: usize,
}

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

pub struct OptimizedChunk<T> {
    permutations_compressed: Vec<FixedArray>,
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

#[derive(Copy, Clone)]
struct OptimizedJob {
    compressed_values: FixedArray,
    compressed_permutation: FixedArray,
    permutation_length: usize,
}

impl OptimizedJob {
    fn new(compressed_values: FixedArray) -> Self {
        Self {
            compressed_values,
            compressed_permutation: zeroed_fixed_array(),
            permutation_length: 0,
        }
    }

    fn compute_next_jobs(self) -> Vec<OptimizedJob> {
        let mut result = vec![];

        for (idx, freq) in self.compressed_values.iter().enumerate() {
            if *freq > 0 {
                result.push(self.with_new_value(&idx))
            }
        }
        result
    }

    fn is_ready(&self) -> bool {
        self.compressed_values.eq(&zeroed_fixed_array())
    }

    fn permutation(self) -> FixedArray {
        self.compressed_permutation
    }

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
}
