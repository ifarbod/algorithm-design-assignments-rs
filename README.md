# algorithm-design-assignments-rs

Algorithm design uni assignments made in Rust. H1/Q2 2023.

The assignments:
- `merge_sort_3`: An implementation of the merge sort algorithm, dividing the array by 3 instead of 2.
- `mst`: Calculating the minimum spanning tree of a graph, with random weights from 10 to 20, using the Kruskal algorithm.

To run them, `cd` to their directory and execute `cargo run`. Alternatively, you can do this:

```shell
$ cargo run --bin mst
```

To run the unit tests:

```shell
$ cargo test --bin tests
```
