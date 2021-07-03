pub mod into_optimized_chunks;
pub mod into_chunks;

use std::hash::Hash;
use into_optimized_chunks::IntoOptimizedChunks;
use into_chunks::IntoChunks;

pub struct Permutations<T: Copy> {
    values: Vec<T>,
}

impl<T: Copy + Eq + Hash + ToString> Permutations<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self { values }
    }
    pub fn into_optimized_chunks(self, size: usize) -> Result<IntoOptimizedChunks<T>, String> {
        if size == 0 {
            panic!("Chunks size must be at least one")
        }
        IntoOptimizedChunks::new(self.values, size)
    }
    pub fn into_chunks(self, size: usize) -> IntoChunks<T> {
        if size == 0 {
            panic!("Chunks size must be at least one")
        }
        IntoChunks::new(self.values, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimized_permutations_are_computed_correctly() {
        let permutations = Permutations::new(vec![1, 2, 3])
            .into_optimized_chunks(2)
            .unwrap()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        assert_eq!(
            permutations.join("\n"),
            "3,2,1\n3,1,2\n\n2,3,1\n2,1,3\n\n1,3,2\n1,2,3\n".to_string()
        );

        let permutations = Permutations::new(vec![1, 2, 2])
            .into_optimized_chunks(2)
            .unwrap()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        assert_eq!(
            permutations.join("\n"),
            "2,2,1\n2,1,2\n\n1,2,2\n".to_string()
        );

        let permutations = Permutations::new(vec![1, 1, 1])
            .into_optimized_chunks(2)
            .unwrap()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        assert_eq!(permutations.join("\n"), "1,1,1\n".to_string());

        let permutations = Permutations::new(vec![1, 2, 3, 4])
            .into_optimized_chunks(100)
            .unwrap()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        assert_eq!(
            permutations.join("\n"),
            "4,3,2,1\n4,3,1,2\n4,2,3,1\n4,2,1,3\n4,1,3,2\n4,1,2,3\n3,4,2,1\n3,4,1,2\n3,2,4,1\n3,2,1,4\n3,1,4,2\n3,1,2,4\n2,4,3,1\n2,4,1,3\n2,3,4,1\n2,3,1,4\n2,1,4,3\n2,1,3,4\n1,4,3,2\n1,4,2,3\n1,3,4,2\n1,3,2,4\n1,2,4,3\n1,2,3,4\n".to_string()
        );
    }

    #[test]
    fn permutations_are_computed_correctly() {
        let mut permutations = Permutations::new(vec![1, 2, 3])
            .into_chunks(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        permutations.sort();
        assert_eq!(
            permutations.join("\n"),
            "1,2,3\n\n1,3,2\n\n2,1,3\n\n2,3,1\n\n3,1,2\n\n3,2,1\n".to_string()
        );

        let mut permutations = Permutations::new(vec![1, 2, 2])
            .into_chunks(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        permutations.sort();
        assert_eq!(
            permutations.join("\n"),
            "1,2,2\n\n2,1,2\n\n2,2,1\n".to_string()
        );

        let mut permutations = Permutations::new(vec![1, 1, 1])
            .into_chunks(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        permutations.sort();
        assert_eq!(permutations.join("\n"), "1,1,1\n".to_string());

        let mut permutations = Permutations::new(vec![1, 2, 3, 4])
            .into_chunks(1)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        permutations.sort();
        assert_eq!(permutations.join("\n"), "1,2,3,4\n\n1,2,4,3\n\n1,3,2,4\n\n1,3,4,2\n\n1,4,2,3\n\n1,4,3,2\n\n2,1,3,4\n\n2,1,4,3\n\n2,3,1,4\n\n2,3,4,1\n\n2,4,1,3\n\n2,4,3,1\n\n3,1,2,4\n\n3,1,4,2\n\n3,2,1,4\n\n3,2,4,1\n\n3,4,1,2\n\n3,4,2,1\n\n4,1,2,3\n\n4,1,3,2\n\n4,2,1,3\n\n4,2,3,1\n\n4,3,1,2\n\n4,3,2,1\n"
            .to_string())
    }
}
