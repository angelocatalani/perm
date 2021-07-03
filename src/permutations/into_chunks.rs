use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

pub struct IntoChunks<T> {
    job_queue: Vec<Job<T>>,
    size: usize,
}
impl<T: Copy + Eq + Hash> IntoChunks<T> {
    pub(crate) fn new(values: Vec<T>, size: usize) -> Self {
        let permutation_length = values.len();
        Self {
            job_queue: vec![Job::new(values_with_frequency(values), permutation_length)],
            size,
        }
    }
}
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
fn values_with_frequency<T: Eq + Hash>(values: Vec<T>) -> HashMap<T, usize> {
    let mut values_with_frequency = HashMap::new();
    for value in values {
        *values_with_frequency.entry(value).or_insert(0) += 1;
    }
    values_with_frequency
}

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

struct Job<T> {
    values_with_positive_frequency: HashMap<T, usize>,
    permutation: Vec<T>,
    permutation_length: usize,
}
impl<T: Copy + Eq + Hash> Job<T> {
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

    fn compute_next_jobs(self) -> Vec<Job<T>> {
        let mut next_jobs = vec![];
        for (value, _) in self.values_with_positive_frequency.iter() {
            let next_job = self.with_new_value(&value);
            next_jobs.push(next_job);
        }
        next_jobs
    }

    fn permutation(self) -> Vec<T> {
        self.permutation
    }

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
    fn is_ready(&self) -> bool {
        self.permutation.len() == self.permutation_length
    }
}

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
