# Permutations

# Table of Contents

* [Algorithm](#algorithm)
* [Complexity](#complexity)
* [Usage](#usage)
* [External Libraries](#external-libraries)

## Algorithm

**optimized chunks vs chunks**

**Why iterative**

**Why chunks**

**Tell what happens with integers instead of string**

**Improvements**

## Complexity

**time**

**space**

## Usage

First clone the repository and enter the project directory:
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

It is possible to run the benchmarks for the two versions of the algorithm,
with the following input: `[1,2,3,4,5,6,7,8,9,10]`
```shell
cargo bench
```

## External Libraries

**tests**

**criterion**

**scoped threads**
