# Learning_Rust
This is a repository where I collect some simple algorithms implemented in Rust. The idea is to learn Rust and refresh on some basic algorithms at the same time. This is an ongoing project. Currently, it contains the following algorithms:

## Sorting Algorithms
Among other things I implement the typical sorting algorithms.

### Quick Sort
This is an implementation of the usual quick sort algorithm. In Rust recursion does not work together with mutable references as function parameters. To circumvent this, we define a struct that takes a mutable reference upon construction and implement the sort-function as a struct-method.

### Merge Sort
An implementation of the merge sort algorithm.


## Miscellaneous

### Erastothenes' sieve
This is an implementation of the classical algorithm to find prime numbers.
