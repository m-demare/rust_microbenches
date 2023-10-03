## A small collection of unrelated rust benchmarks I had to write

Benchmarks can be run with `cargo bench [bench_name]`
(or you can run all with `cargo bench`)


### HashMap vs Vec for lookup and insertion (`bench_find, bench_add`)

For bigger sizes, it's obvious a HashMap wins, thanks to its O(1) average lookup and
insertion times (both of which are O(n) in an unsorted Vec, assuming an insertion
also requires a lookup, to guarantee unique keys), but I needed to know what happened
with smaller maps.

I specifically needed to test this with `char` as keys, which are extremely quick to
compare by equality. In particular, I wouldn't have many possible values, only
`[a-zA-Z0-9]`, and in general a small subset of those. The size of the values is also
rather small (just an u32)

The code for the benchmarks can be found [here](https://github.com/m-demare/rust_microbenches/blob/main/benches/small_map_bench.rs)

My results were the following:

![bench_find](https://github.com/m-demare/rust_microbenches/blob/main/img/bench_find.svg?raw=true)

![bench_add](https://github.com/m-demare/rust_microbenches/blob/main/img/bench_add.svg?raw=true)

Up to about 40 items, a Vec is faster than a HashMap for lookup, and at least up to
60 items, a Vec is also faster for insertion

Additionally, FxHashMaps are an alternative implementation of HashMaps, with a faster
hashing algorithm. They seem to be considerably faster than HashMaps, and they
overtake Vecs at ~10 items for lookup, and ~40 for insertion

