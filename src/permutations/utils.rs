use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

pub fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        factorial(n - 1) * n
    }
}

/// Compute the hashmap with the frequency for each value.
pub fn values_with_frequency<T: Copy + Eq + Hash>(values: &[T]) -> HashMap<T, usize> {
    let mut values_with_frequency = HashMap::new();
    for value in values {
        *values_with_frequency.entry(*value).or_insert(0) += 1;
    }
    values_with_frequency
}

/// Decrease the frequency of `value` from `values_with_frequency`,
/// and it deletes the new entry if the resulting frequency is zero.
pub fn decrease_or_remove_positive_frequency<T: Copy + Hash + Eq>(
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
