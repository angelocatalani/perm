//! # Permutation
//!
//! Parse the input string and generate the iterator over chunks of permutations.

use std::convert::TryFrom;
use std::hash::Hash;

use into_chunks::IntoChunks;
use into_optimized_chunks::IntoOptimizedChunks;

use crate::permutations::into_optimized_chunks::PERMUTATION_FIXED_LENGTH;

pub mod into_chunks;
pub mod into_optimized_chunks;

/// Permutations.
pub struct Permutations<T: Copy> {
    values: Vec<T>,
}

impl<T: Copy + Eq + Hash> Permutations<T> {
    /// Initialize a new `Permutations` with the values to permute.
    pub fn new(values: Vec<T>) -> Self {
        Self { values }
    }
    /// Compute the length of each permutation.
    pub fn length(&self) -> usize {
        self.values.len()
    }
    /// Check if the input values is short enough to use the optimized version of the algorithm.
    pub fn can_be_optimized(&self) -> bool {
        self.values.len() <= PERMUTATION_FIXED_LENGTH
    }
    /// Create the optimized iterator over chunks of permutations.
    /// Panics if the chunk size is zero
    /// or the input values are not short enough to use the optimized version of the algorithm.
    pub fn into_optimized_chunks(self, size: usize) -> IntoOptimizedChunks<T> {
        if size == 0 {
            panic!("Chunks size must be at least one")
        }
        if !self.can_be_optimized() {
            panic!("Cannot use optimized_chunks because the permutation is: `{}` and the maximum length is: {}", self.values.len(), PERMUTATION_FIXED_LENGTH)
        }
        IntoOptimizedChunks::new(self.values, size)
    }
    /// Create the slower iterator over chunks of permutations
    /// with no limitations of permutation length.
    pub fn into_chunks(self, size: usize) -> IntoChunks<T> {
        if size == 0 {
            panic!("Chunks size must be at least one")
        }
        IntoChunks::new(self.values, size)
    }
}

/// Initialize the `Permutations` from a given string.
/// It fails if the input is not a string of comma separated numbers.
impl<'a> TryFrom<&'a str> for Permutations<&'a str> {
    type Error = String;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        text.split(',')
            .try_fold(Vec::new(), |acc, number| {
                if number.trim().parse::<f64>().is_ok() {
                    let mut new_vec = acc.clone();
                    new_vec.push(number.trim());
                    Ok(new_vec)
                } else {
                    Err(format!("`{}` is not a valid number", number))
                }
            })
            .map(Permutations::new)
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;
    use fake::Faker;
    use itertools::Itertools;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;

    use super::*;

    #[derive(Clone, Debug)]
    struct RandomIntegersWithDuplicates(Vec<i32>);

    impl Arbitrary for RandomIntegersWithDuplicates {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let random_values: [i32; 5] = Faker.fake_with_rng(g);
            let mut values = random_values.to_vec();
            (0..2).for_each(|_| values.push(random_values[rand::thread_rng().gen_range(0..4)]));
            Self(random_values.to_vec())
        }
    }

    #[derive(Clone, Debug)]
    struct RandomStringsWithDuplicates(Vec<String>);

    impl Arbitrary for RandomStringsWithDuplicates {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let random_values: [String; 5] = Faker.fake_with_rng(g);
            let mut values = random_values.to_vec();
            (0..2).for_each(|_| {
                values.push(random_values[rand::thread_rng().gen_range(0..4)].clone())
            });
            Self(random_values.to_vec())
        }
    }

    #[derive(Clone, Debug)]
    struct InvalidInput(String);

    impl Arbitrary for InvalidInput {
        fn arbitrary<G: Gen>(_g: &mut G) -> Self {
            let invalid_inputs = ["", "c", "+123,", "123,345,,45"];
            Self(invalid_inputs[rand::thread_rng().gen_range(0..invalid_inputs.len())].to_string())
        }
    }

    #[derive(Clone, Debug)]
    struct ValidInput(String);

    impl Arbitrary for ValidInput {
        fn arbitrary<G: Gen>(_g: &mut G) -> Self {
            let valid_inputs = [
                "1",
                "1\n",
                "1,2",
                "1,2\n",
                "-1",
                "+1,-1",
                "1.12",
                "-1.23,+12",
            ];
            Self(valid_inputs[rand::thread_rng().gen_range(0..valid_inputs.len())].to_string())
        }
    }

    fn generate_correct_permutations<T: ToString + PartialEq + Eq + Hash>(
        values: Vec<T>,
    ) -> Vec<String> {
        let correct_permutations = values
            .iter()
            .permutations(values.len())
            .dedup()
            .unique()
            .collect::<Vec<Vec<&T>>>();

        let mut correct_permutations_strings = correct_permutations
            .iter()
            .map(|p| {
                p.iter()
                    .take(p.len() - 1)
                    .fold(String::new(), |acc, x| acc + &x.to_string() + ",")
                    + &p[p.len() - 1].to_string()
                    + "\n"
            })
            .collect::<Vec<String>>();
        correct_permutations_strings.sort();
        correct_permutations_strings
    }

    #[quickcheck_macros::quickcheck]
    fn invalid_input_is_detected(invalid_input: InvalidInput) {
        assert!(Permutations::try_from(invalid_input.0.as_str()).is_err());
    }

    #[quickcheck_macros::quickcheck]
    fn valid_input_is_parsed_correctly(valid_input: ValidInput) {
        let permutations = Permutations::try_from(valid_input.0.as_str()).unwrap();
        let mut permutation_strings = permutations
            .into_chunks(1)
            .map(|p| p.to_string())
            .collect::<Vec<String>>();
        permutation_strings.sort();

        let tokens = valid_input
            .0
            .split(',')
            .map(|t| t.trim().to_string())
            .collect::<Vec<String>>();
        let mut expected_permutations = generate_correct_permutations(tokens);
        expected_permutations.sort();
        assert_eq!(permutation_strings, expected_permutations)
    }

    #[quickcheck_macros::quickcheck]
    fn permutations_of_integers_are_computed_correctly(values: RandomIntegersWithDuplicates) {
        let mut permutations = Permutations::new(values.0.clone())
            .into_chunks(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        permutations.sort();
        assert_eq!(permutations, generate_correct_permutations(values.0))
    }

    #[quickcheck_macros::quickcheck]
    fn permutations_of_strings_are_computed_correctly(values: RandomStringsWithDuplicates) {
        let mut permutations = Permutations::new(values.0.iter().map(|v| v.as_str()).collect())
            .into_chunks(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        permutations.sort();
        assert_eq!(permutations, generate_correct_permutations(values.0))
    }

    #[quickcheck_macros::quickcheck]
    fn optimized_permutations_of_integers_are_computed_correctly(
        values: RandomIntegersWithDuplicates,
    ) {
        let mut optimized_permutations = Permutations::new(values.0.clone())
            .into_optimized_chunks(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        optimized_permutations.sort();
        assert_eq!(
            optimized_permutations,
            generate_correct_permutations(values.0)
        )
    }

    #[quickcheck_macros::quickcheck]
    fn optimized_permutations_of_strings_are_computed_correctly(
        values: RandomStringsWithDuplicates,
    ) {
        let mut optimized_permutations =
            Permutations::new(values.0.iter().map(|v| v.as_str()).collect())
                .into_optimized_chunks(1)
                .map(|c| c.to_string())
                .collect::<Vec<String>>();
        optimized_permutations.sort();
        assert_eq!(
            optimized_permutations,
            generate_correct_permutations(values.0)
        )
    }

    #[test]
    fn empty_permutation_is_computed_correctly() {
        let permutations = Permutations::<i32>::new(vec![])
            .into_chunks(2)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        let optimized_permutations = Permutations::<i32>::new(vec![])
            .into_optimized_chunks(2)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        assert!(permutations.is_empty());
        assert!(optimized_permutations.is_empty());
    }

    #[test]
    fn optimized_permutations_of_128_duplicates_are_computed_correctly() {
        let permutations = Permutations::new([0; 128].to_vec())
            .into_optimized_chunks(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        assert_eq!(
            permutations.join(""),
            std::iter::repeat("0,").take(127).collect::<String>() + "0\n"
        );
    }

    #[test]
    fn optimized_permutations_of_129_values_cannot_be_computed() {
        let permutations = Permutations::new([0; 129].to_vec());
        assert!(!permutations.can_be_optimized());
    }

    #[test]
    fn long_permutations_are_computed_correctly() {
        let permutations = Permutations::new([0; 129].to_vec())
            .into_chunks(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        assert_eq!(
            permutations.join(""),
            std::iter::repeat("0,").take(128).collect::<String>() + "0\n"
        );
    }
}
