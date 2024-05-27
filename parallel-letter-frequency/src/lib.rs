use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

type Frequency = HashMap<char, usize>;

pub fn frequency(input: &[&str], worker_count: usize) -> Frequency {
    frequency_v4(input, worker_count)
}

// Simplified version without Arc and Mutex, and using thread scope instead
// Based on solution from https://exercism.org/tracks/rust/exercises/parallel-letter-frequency/solutions/cuckookernel
//
// Benchmark results:
// test bench_large_parallel   ... bench:     100,490.37 ns/iter (+/- 3,484.93)
// test bench_large_sequential ... bench:     233,642.01 ns/iter (+/- 3,289.75)
// test bench_small_parallel   ... bench:      50,563.50 ns/iter (+/- 715.21)
// test bench_small_sequential ... bench:       8,310.77 ns/iter (+/- 69.68)
// test bench_tiny_parallel    ... bench:      18,152.98 ns/iter (+/- 898.80)
// test bench_tiny_sequential  ... bench:          37.26 ns/iter (+/- 0.40)
pub fn frequency_v2(input: &[&str], worker_count: usize) -> Frequency {
    thread::scope(|s| {
        if input.is_empty() || worker_count == 0 {
            return HashMap::new();
        }

        let mut handles = Vec::with_capacity(worker_count);

        // Calculate the rounded-up division result
        let chunk_size = input.len() / worker_count + 1;

        for chunk in input.chunks(chunk_size) {
            let handle = s.spawn(|| count_frequency(chunk));

            handles.push(handle);
        }

        let mut freq = Frequency::new();
        for handle in handles {
            let thread_freq = handle.join().expect("Thread handler failed");

            for (c, count) in thread_freq {
                let entry = freq.entry(c).or_default();
                *entry += count;
            }
        }

        freq
    })
}

// Third version, more functional approach
pub fn frequency_v3(input: &[&str], worker_count: usize) -> Frequency {
    thread::scope(|s| {
        if input.is_empty() || worker_count == 0 {
            return HashMap::new();
        }

        let mut freq = Frequency::new();
        let chunk_size = input.len() / worker_count + 1;

        let mut handles = input
            .chunks(chunk_size)
            .map(|chunk| s.spawn(|| count_frequency(chunk)))
            .collect::<Vec<thread::ScopedJoinHandle<_>>>();

        handles
            .drain(..)
            .map(|h| h.join().expect("Couldn't join handler"))
            .flat_map(|freq| freq.into_iter())
            .for_each(|(c, count)| {
                freq.entry(c)
                    .and_modify(|total| *total += count)
                    .or_insert(count);
            });

        freq
    })
}

// Forth version, functional approach all in single chain flow
pub fn frequency_v4(input: &[&str], worker_count: usize) -> Frequency {
    thread::scope(|s| {
        if input.is_empty() || worker_count == 0 {
            return HashMap::new();
        }

        // Calculate the rounded-up division result
        let chunk_size = input.len() / worker_count + 1;

        let mut freq = Frequency::new();

        input
            .chunks(chunk_size)
            .map(|chunk| s.spawn(|| count_frequency(chunk)))
            .collect::<Vec<thread::ScopedJoinHandle<_>>>()
            .drain(..)
            .map(|h| h.join().expect("Couldn't join handler"))
            .flat_map(|freq| freq.into_iter())
            .for_each(|(c, count)| {
                freq.entry(c)
                    .and_modify(|total| *total += count)
                    .or_insert(count);
            });

        freq
    })
}

fn count_frequency(input: &[&str]) -> Frequency {
    let mut freq = Frequency::new();

    input.iter().for_each(|l| {
        l.chars().for_each(|c| {
            if !c.is_alphabetic() {
                return;
            }

            let lc = c.to_ascii_lowercase();

            let entry = freq.entry(lc).or_insert(0);
            *entry += 1;
        });
    });

    freq
}

// Fist version built on my own trying to figure out data access on
// threads from the main function, which relies on Arc and Mutex.
//
// Benchmark results:
// test bench_large_parallel   ... bench:     311,331.51 ns/iter (+/- 10,447.76)
// test bench_large_sequential ... bench:     232,769.79 ns/iter (+/- 2,786.92)
// test bench_small_parallel   ... bench:      52,028.84 ns/iter (+/- 2,054.97)
// test bench_small_sequential ... bench:       8,320.24 ns/iter (+/- 59.94)
// test bench_tiny_parallel    ... bench:      17,485.83 ns/iter (+/- 1,473.71)
// test bench_tiny_sequential  ... bench:          38.09 ns/iter (+/- 0.40)
pub fn frequency_v1(input: &[&str], worker_count: usize) -> Frequency {
    if input.len() == 0 || worker_count == 0 {
        return HashMap::new();
    }

    let freq = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    // Calculate the rounded-up division result
    let chunk_size = (input.len() + worker_count - 1) / worker_count;

    for chunk in input.chunks(chunk_size) {
        // As input is composed of &str items we need to make a copy
        // of each &str into a String so its data content can be accessed
        // from the thread scope without requiring static lifetime
        // for the &str data.
        let worker_lines: Vec<String> = chunk.iter().map(|l| String::from(l.to_owned())).collect();

        let freq = Arc::clone(&freq);
        let handle = thread::spawn(move || {
            let mut f = freq.lock().unwrap();

            worker_lines.iter().for_each(|l| {
                l.chars().for_each(|c| {
                    if !c.is_alphabetic() {
                        return;
                    }

                    let lc = c.to_lowercase().nth(0).unwrap();

                    let entry = f.entry(lc).or_insert(0);
                    *entry += 1;
                });
            });
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Error joining handler");
    }

    let lock = Arc::try_unwrap(freq).expect("Lock still has multiple owners");
    lock.into_inner().expect("Mutex cannot be locked")
}
