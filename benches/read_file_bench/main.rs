mod utils;

use std::{path::PathBuf, fs::File, io::{self, BufRead, Read}, time::Duration};
use utf8_chars::BufReadCharsExt;
use core::str::from_utf8;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

use utils::{read_with_buf, CharReader1, CharReader2, ByteReader, consume};

fn bench_as_str(c: &mut Criterion, name: &str, file: PathBuf) {
    let mut group = c.benchmark_group("read_file_as_str");

    group.bench_with_input(BenchmarkId::new(name, "read_to_string"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            black_box(std::fs::read_to_string(i)?);
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "bufreader_lines"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            for l in io::BufReader::new(File::open(i)?).lines() {
                black_box(l?);
            }
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "bufreader_read_lines"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            let mut line = String::new();
            let mut reader = io::BufReader::new(File::open(i)?);
            while let Ok(n) = reader.read_line(&mut line) {
                if n==0 {break;}
                black_box(line.len());
            }
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "bufreader_read_8k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            read_with_buf!(io::BufReader::new(File::open(i)?), 2usize.pow(13))
        }));

    group.bench_with_input(BenchmarkId::new(name, "file_read_2k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            read_with_buf!(File::open(i)?, 2usize.pow(11))
        }));

    group.bench_with_input(BenchmarkId::new(name, "file_read_4k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            read_with_buf!(File::open(i)?, 2usize.pow(12))
        }));

    group.bench_with_input(BenchmarkId::new(name, "file_read_8k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            read_with_buf!(File::open(i)?, 2usize.pow(13))
        }));

    group.bench_with_input(BenchmarkId::new(name, "file_read_16k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            read_with_buf!(File::open(i)?, 2usize.pow(14))
        }));

    group.bench_with_input(BenchmarkId::new(name, "file_read_32k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            read_with_buf!(File::open(i)?, 2usize.pow(15))
        }));
}

fn bench_as_chars(c: &mut Criterion, name: &str, file: PathBuf) {
    let mut group = c.benchmark_group("read_file_as_chars");
    group.sample_size(20);
    group.bench_with_input(BenchmarkId::new(name, "buf_reader_bytes"), &file, 
        // Tried it with a 16k buffer too, both are too slow,
        // made benchmarks a pain to run so I left just one
        |b, i| b.iter(|| -> io::Result<()> {
            let reader = io::BufReader::new(File::open(i)?);
            consume(reader.bytes());
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "read_to_string_chars"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            let s = std::fs::read_to_string(i)?;
            consume(s.chars());
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "file_read_chars_8k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            read_with_buf!(File::open(i)?, 2usize.pow(13), true)
        }));

    group.bench_with_input(BenchmarkId::new(name, "file_read_chars_16k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            read_with_buf!(File::open(i)?, 2usize.pow(14), true)
        }));

    group.bench_with_input(BenchmarkId::new(name, "char_reader_8k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            consume(CharReader1::with_capacity(2usize.pow(13), File::open(i)?));
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "char_reader_16k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            consume(CharReader1::new(File::open(i)?));
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "char_reader2_8k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            consume(CharReader2::with_capacity(2usize.pow(13), File::open(i)?));
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "char_reader2_16k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            consume(CharReader2::new(File::open(i)?));
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "byte_reader_16k"), &file, 
        |b, i| b.iter(|| -> io::Result<()> {
            consume(ByteReader::new(File::open(i)?));
            Ok(())
        }));

    group.bench_with_input(BenchmarkId::new(name, "utf8_chars_crate_8k"), &file, 
        // Tried it with a 16k buffer too, both are too slow,
        // made benchmarks a pain to run so I left just one
        |b, i| b.iter(|| -> io::Result<()> {
            let mut reader = io::BufReader::new(File::open(i)?);
            consume(reader.chars());
            Ok(())
        }));
}

fn short_file(c: &mut Criterion){
    bench_as_str(c, "short_file", PathBuf::from("benchfiles/short_file"));
    bench_as_chars(c, "short_file", PathBuf::from("benchfiles/short_file"));
}

fn long_file_short_lines(c: &mut Criterion){
    bench_as_str(c, "long_file_short_lines", PathBuf::from("benchfiles/short_lines"));
    bench_as_chars(c, "long_file_short_lines", PathBuf::from("benchfiles/short_lines"));
}

fn long_file_long_lines(c: &mut Criterion){
    bench_as_str(c, "long_file_long_lines", PathBuf::from("benchfiles/long_lines"));
    bench_as_chars(c, "long_file_long_lines", PathBuf::from("benchfiles/long_lines"));
}

criterion_group! {
    name = read_file_bench;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2));
    targets =
        short_file,
        long_file_short_lines,
        long_file_long_lines,
}
criterion_main!(read_file_bench);

