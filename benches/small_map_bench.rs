use std::{collections::HashMap, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::{seq::{SliceRandom, IteratorRandom}, SeedableRng, rngs::StdRng, Rng};
use rustc_hash::FxHashMap;

static SIZES: [usize; 10] = [2, 5, 10, 15, 20, 25, 30, 40, 50, 60];

fn bench_find(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(123);

    let mut group = c.benchmark_group("bench_find");
    for size in SIZES {
        let mut vec = Vec::new();
        let mut hash = HashMap::new();
        let mut fxhash = FxHashMap::default();
        let mut input = get_input(&mut rng, size);
        for ch in input.iter() {
            let val: u32 = rng.gen();
            vec.push((*ch, val));
            hash.insert(*ch, val);
            fxhash.insert(*ch, val);
        }
        input.shuffle(&mut rng);

        group.bench_with_input(BenchmarkId::new("Vec", size), &input, 
            |b, i| b.iter(|| 
                for ch in black_box(i.iter()) {
                    black_box(vec.iter().find(|el| el.0==*ch));
                }
            ));
        group.bench_with_input(BenchmarkId::new("Hash", size), &input, 
            |b, i| b.iter(|| 
                for ch in black_box(i.iter()) {
                    black_box(hash.get(ch));
                }
            ));
        group.bench_with_input(BenchmarkId::new("FxHash", size), &input, 
            |b, i| b.iter(|| 
                for ch in black_box(i.iter()) {
                    black_box(fxhash.get(ch));
                }
            ));
    }
}

fn bench_add(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(123);

    let mut group = c.benchmark_group("bench_add");
    for size in SIZES {
        let mut input = get_input(&mut rng, size);
        input.shuffle(&mut rng);

        group.bench_with_input(BenchmarkId::new("Vec", size), &input, 
            |b, i| b.iter(|| {
                let mut vec = Vec::new();
                for ch in i.iter() {
                    if !vec.iter().any(|(c, _)| ch==c) {
                        vec.push((*ch, black_box(0)));
                    }
                }
            }));
        group.bench_with_input(BenchmarkId::new("Hash", size), &input, 
            |b, i| b.iter(|| {
                let mut map = HashMap::new();
                for ch in i.iter() {
                    map.insert(*ch, black_box(0));
                }
            }));
        group.bench_with_input(BenchmarkId::new("FxHash", size), &input, 
            |b, i| b.iter(|| {
                let mut map = FxHashMap::default();
                for ch in i.iter() {
                    map.insert(*ch, black_box(0));
                }
            }));
    }
}

fn get_input(rng: &mut StdRng, size: usize) -> Vec<char> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789".chars().collect();
    if size > alphabet.len() { panic!("Max size {}", alphabet.len()); }
    alphabet.into_iter()
        .choose_multiple(rng, size)
        .to_vec()
}

criterion_group! {
    name = small_map_bench;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3));
    targets = bench_find, bench_add,
}
criterion_main!(small_map_bench);

