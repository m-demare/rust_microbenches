# A small collection of unrelated rust benchmarks I had to write

Benchmarks can be run with `cargo bench [bench_name]`
(or you can run all with `cargo bench`)


## HashMap vs Vec for lookup and insertion (`bench_find, bench_add`)

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

![bench_find](https://github.com/m-demare/rust_microbenches/blob/main/img/bench_find.jpg?raw=true)

![bench_add](https://github.com/m-demare/rust_microbenches/blob/main/img/bench_add.jpg?raw=true)

Up to about 40 items, a Vec is faster than a HashMap for lookup, and at least up to
60 items, a Vec is also faster for insertion

Additionally, FxHashMaps are an alternative implementation of HashMaps, with a faster
hashing algorithm. They seem to be considerably faster than HashMaps, and they
overtake Vecs at ~10 items for lookup, and ~40 for insertion


## File reads (`read_file_as_str, read_file_as_chars`)

### Reading as string

I needed to know the fastest way to read files of different sizes. The benchmarked
options were:

- read_to_string: `std::fs::read_to_string`
- bufreader_lines: `io::BufReader::lines` (reads line by line)
- bufreader_read_lines: `io::BufReader::read_line` (reuses allocated string, reads line by line)
- bufreader_read_X: `io::BufReader::read` (reads as much as possible, within a given buffer, using a BufReader. X is the buf size)
- file_read_X: `std::fs::File::read` (reads as much as possible, within a given buffer, directly from the file. X is the buf size)

The results were the following:

![short_file_as_str](https://github.com/m-demare/rust_microbenches/blob/main/img/short_file_as_str.jpg?raw=true)

![long_file_short_lines_as_str](https://github.com/m-demare/rust_microbenches/blob/main/img/long_file_short_lines_as_str.jpg?raw=true)

![long_file_long_lines_as_str](https://github.com/m-demare/rust_microbenches/blob/main/img/long_file_long_lines_as_str.jpg?raw=true)

Using std::fs::File::read with a 16k buffer seems to be the sweet spot, though
read_to_string is surprisingly decent considering how simple it is to use.
Looking at its implementation, this is probably because it looks at the file's
metadata, and preallocates the exact size it needs for the file.
Allocating that much memory is quite slow, but it only needs to make one single
allocation.

One possible issue that may harm the accuracy of these results is caching, I don't
know if there's any way to clean the fs's cache for one file

### Reading as chars

Actually the main reason why I did this, is that I needed to iterate a file by
chars. I thought reading substrings would give me a reasonable idea of the expected
performance, but then I wrote some more benchmarks and I was surprised. The
benchmarked options here were:

- read_to_string_chars: `std::fs::read_to_string::chars`
- buf_reader_bytes: `io::BufReader::bytes`
- byte_reader_16k: A simple hand-made implementation of a buffered iterator over u8
- file_read_chars_X: Reads file by chunks, converts to string, and iterates over its
  chars. Attempts to handle error caused by splitting a utf8 char in half
- char_reader_X: Same as previous, but turned into an Iterator<Item = char>
- char_reader2_X: Similar to byte_reader_X, but converts u8's to chars before
returning them (doesn't handle utf8 chars bigger than 1 byte, but wouldn't be too
hard to implement)
- utf8_chars_crate_X: The implementation provided by the utf8-chars crate

Results were the following:

![short_file_as_chars](https://github.com/m-demare/rust_microbenches/blob/main/img/short_file_as_chars.jpg?raw=true)

![long_file_short_lines_as_chars](https://github.com/m-demare/rust_microbenches/blob/main/img/long_file_short_lines_as_chars.jpg?raw=true)

![long_file_long_lines_as_chars](https://github.com/m-demare/rust_microbenches/blob/main/img/long_file_long_lines_as_chars.jpg?raw=true)

I'm quite surprised by the difference between file_read_chars_X and char_reader_X,
because unless I made some dumb mistake, that difference can only be attributed to
the iterator's overhead. But in the context of file reading, some function calls
shouldn't be able to cause a 3x-4x difference in time

It also surprised me how slow `io::BufReader::bytes` seems to be, compared to the
rather simple implementation I wrote

