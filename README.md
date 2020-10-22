# benchmark_sampledata

If you need collections of files to run your benchmarks on, this repo/crate might be of use.


```rust

use benchmark_sampledata;

let sd = benchmark_sampledata::linux_kernel().unwrap();


```

This crate will download and decompress source files such as the linux kernel or the cargo source tree. You can do what you want with these files afterwards, such as run real-world performance benchmarks. Please open an issue if you want other sample data included.

Docs: https://docs.rs/benchmark_sampledata/
![Rust](https://github.com/woelper/benchmark_sampledata/workflows/Rust/badge.svg)
