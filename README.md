# Permutations

# Table of Contents

* [Algorithm](#algorithm)
* [Complexity](#complexity)
* [Usage](#usage)
* [External Libraries](#external-libraries)

## Algorithm

This library can generate permutations with an optimized iterator or a normal iterator.

They both share the same algorithm, but the optimized iterator is more efficient because it limits the usage of heap
allocated data. It uses generic fixed arrays of length: `128` to store a permutation. Fixed array are stack allocated,
and copied efficiently.

The algorithm is iterative to avoid the overhead of stack frames due to the recursive function calls.

The permutations are computed in chunks, so that it is possible to delegate the I/O operations to a new thread. In this
way the remaining permutations can be computed without waiting for blocking I/O to terminate.

To use fixed array to store the permutations, the optimized iterator convert the input numbers to:

- `compressed_array`
- `hash_map`

The `compressed_array` is such that at index: `i`, `compressed_array[i]`
is the frequency of `hash_map[i]` in the original input. In particular, `hash_map` has as key a number of the input, and
as value the index of that number in `compressed_array`.

## Algorithm Steps

Suppose to have as input: `Input = [123,234,234]`.

To avoid duplicates, we generate from the input a map:
`root_map`, where the key is the number, and the value is the frequency.

`root_map: {123:1,234:2}`.

Then, we create the `root_job` with the `root_map` and an empty permutation: `[]`.

`root_job: {root_map,`[]`}`

Then, we add the `root_job` to the job queue list: `job_queue`.

`job_queue = [ root_job ]`

Until the `job_queue` is empty:

- **Step-0**: pop the last (order is relevant only for the complexity of the operation itself)
  element of `job_queue`: `job_0`

- **Step-1**: for each key: `k_i` of the `map_0` with non-zero frequency: `f_i>0`:
    - generate a new job: `job_i` with `permutation_i=k_i+permutation_0`, and a new map: `map_i` that is the same
      of `map_0` but with `f_i` decreased by one.

    - push `job_i` to `job_queue` if `len(permutation_i)<len(Input)` otherwise `permutation_i` is a new valid
      permutation.

After the first iterator we have:
`job_queue = [ job_1,job_2]`
where:

- `job_1`= `{123:0,234:2} , [123]`
- `job_2`= `{123:1,234:1} , [234]`.

At the second iteration:
`job_queue = [ job_1,job_3,job_4 ]`
where:

- `job_3`= `{123:0,234:1} , [234,123]`
- `job_4`= `{123:1,234:0} , [234,234]`.

At the third iteration:
`job_queue = [ job_1,job_3 ]` and the permutation: `[234,234,123]` is generated.

At the fourth iteration:
`job_queue = [ job_1 ]` and the permutation: `[234,123,234]` is generated.

At the fifth iteration:
`job_queue = [ job_7 ]` where:

- `job_7`= `{123:0,234:1} , [123,234]`

At the sixth iteration:
`job_queue = []` and the permutation: `[123,234,234]` is generated.

The order of the hash map iterator can be made determinist sorting the key, but it comes to a performance cost
of: `O(log(n)*n)`.

## What happens with integers instead of string

The implementation of the algorithms is generic to the type of input values.

However, if the `Permutations` are built from a strings with the `TryFrom` trait, the type of the input value is a
string (specifically a string slice).

If the `TryFrom` had to parse integers instead of strings, the remaining code (except for the `TryFrom`), would stay the
same because the code is generic over the type of the number type.

In particular, to generate a permutation the input numbers are constrained to be: `Copy+Eq+Hash+ToString`.

It should be possible to make also the `TryFrom` code generic, however, the compilation fails due to conflicting
implementations of the `TryFrom` trait.

## Complexity

With respect to the section: `## Algorithm Steps` we can see that:

- initially the list is empty.
- at each iteration a parent job is removed from the list, and some children jobs are added.

The number of children jobs added at each iteration depends only on the parent job and specifically on the keys
contained in the map of the parent job with positive frequency.

When the input of size: `N` has `N` distinct values, we are in the worst case scenario, because duplicates reduce the
distinct permutations.

The root job has `N` distinct keys in the map: this means after the first iteration,
`N` jobs are added to the `job_queue`.

Each of those children will have `N-1` distinct keys, and it will generate `N-1` more jobs to add to the queue.

Overall those children will add `N*(N-1)` jobs to the list.

This process goes on until `N=1`.

## Time Complexity

The number of times the jobs added to the `job_queue` dominates the time complexity.

To add a job we pay a constant cost: `c=O(1)`.

If we denote with `T(n)` the number of jobs added to the list with `n` distinct values, we have the following recurrence
expression:
`T(n)=n+n*T(n-1)=n+n*(n-1)+n(n-1)(n-2)+...+1, T(1)=c` that can be solved with `WolframAlpha`:
`T(n)=n!*c'`, where `c'` is a constant.

This means the time complexity is: `O(n!)`.

## Space Complexity

This is dominated by the size of the `job_queue`.

At each step we remove one job, and we add the children jobs until a permutation is found.

This means if we have `N` distinct elements:

- initially the queue has `1` job that is the `root_job`.
- then `N`
- then `N-1+N-1 = 2N-(1+1)`
- then `N-1+N-2+N-2 = 3N-(1+2+2)`
- then `N-1+N-2+N-3+N-3 = 4N-(1+2+3+3)`
- then `N-1+N-2+N-3+N-4+N-4 = 5N-(1+2+3+4+4)`

At the `i-th` iteration we have:
`job_queue_length(i) = i*n - (1+2+3+...+(i-1) + (i-1)) = i*n - (i-1)*(i-2)/2 - (i-1)`.

At the last iteration, with `i=N`:
`job_queue_length(N)=N^2-(N-1)(N-2)/2-N+1`.

This means the space complexity is `O(n^2)`. Unfortunately, it is more complex than the classical recursive algorithms
that has a linear, space complexity since it does not need to add the stack frames of all the children nodes.

## Usage

Clone the repository and enter the project directory:

```shell
git clone git@github.com:angelocatalani/perm.git
cd perm
```

The executable reads from `stdin` and writes to `stdout`.

Given an `input` file of comma separated numbers, the following command:

```shell
 cat input | cargo run --release > output
```

writes to `output` all the permutations.

Run the benchmarks for the two versions of the algorithm, with the following input: `[1,2,3,4,5,6,7,8,9,10]`

```shell
cargo bench
```

Visualize the code documentation:

```shell
cargo doc --open --document-private-items
```

## External Libraries

To run tests, I used:

- `quickcheck` and `quickcheck_macros` to run property test
- `fake` and `rand` to generate random data
- `itertools` to get correct permutations to test against my algorithm

To benchmark the code: `criterion`.

Finally, to run code concurrently in the main entry point I used:

- `crossbeam`.

In particular, the main entry point spawns a new thread to output each chunk.

Since, the chunks contain a slice string that is a reference to the user input, they do not have the `'static` lifetime
that is required by the threads of the standard library.

`crossbeam` allows using threads with a lifetime scoped to a given block without the `'static` constraint.

I could avoid the `'static` constraint by using `String` instead of `&str` inside the `chunks`.

However, this would have required to constraint the permutation elements to be `Clone` and use `.clone()` to copy them.

This would result to performance degradation. (probably minimal since chunks are already heap allocated).
