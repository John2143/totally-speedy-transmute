# Totally Speedy Transmute

An evil spiritual successor to [Totally Safe Transmute](https://github.com/ben0x539/totally-safe-transmute/blob/master/src/lib.rs)

## What is it?

Totally Speedy Transmute (TST) is a library providing a small, performance oriented, safe version of `std::mem::transmute`.
TST is intended for:

 - `Totally Safe Transmute` users looking for a windows-compatible alternative.
 - Programmers working in `#![forbid(unsafe_code)]` codebases who dislike their co-workers.
 - C, C++, and Go authors looking write scathing articles about how Rust isn't *actually* memory safe.
 - [IOCCC](https://www.ioccc.org/) winners with an interest in Rust

TST's benefits:

 - no unsafe code
 - no file access to /proc/self/mem
 - pure rust implementation
 - gluten free, vegan
 - no dependencies
 - it would compile on stable if I knew how to use `doc_include`
 - available on windows, linux, and select Samsung 4-Door Smart Refrigerators
 - rated R for violence and language
 - probably `no_std` compatible

For exotic use cases (such as FFI (just RIIR (lol))), or, if you just want to peel back the curtain, see [`safe!`](macro@safe).

## robot testimonial

Proof there is no unsafe:

```text

$ grep unsafe
src/lib.rs
1:#![forbid(unsafe_code)]

safe/src/lib.rs
1:#![forbid(unsafe_code)]

$ cargo geiger
   Compiling safe v0.1.0 (/Users/jschmidt/totally-speedy-transmute/safe)
    Checking totally-speedy-transmute v0.1.0 (/Users/jschmidt/totally-speedy-transmute)
    Finished dev [unoptimized + debuginfo] target(s) in 1.36s
    Scanning done

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    🔒  = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ❓  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    ☢️   = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      🔒 totally-speedy-transmute 0.1.0
0/0        0/0          0/0    0/0     0/0      🔒 └── safe 0.1.0

0/0        0/0          0/0    0/0     0/0

$ cargo t
   Compiling totally-speedy-transmute v0.1.0 (/Users/jschmidt/totally-speedy-transmute)
    Finished test [unoptimized + debuginfo] target(s) in 0.26s
     Running unittests (target/debug/deps/totally_speedy_transmute-c545620424772de5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/wat.rs (target/debug/deps/wat-527d75fb46642aee)

running 5 tests
test leek ... ok
test fast_vec ... ok
test oops_all_mutable ... ok
test safe ... ok
test transmute_u8 ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests totally-speedy-transmute

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


$ cargo t --release
   Compiling totally-speedy-transmute v0.1.0 (/Users/jschmidt/totally-speedy-transmute)
    Finished release [optimized] target(s) in 0.28s
     Running unittests (target/release/deps/totally_speedy_transmute-5a097808724739b1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/wat.rs (target/release/deps/wat-20a81a817034b78e)

running 5 tests
test leek ... ok
test fast_vec ... ok
test transmute_u8 ... ok
test safe ... ok
test oops_all_mutable ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests totally-speedy-transmute

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ pushd ~/cargo-osha/
$ cargo run -- ~/totally-speedy-transmute/**/*.rs
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cargo-osha /Users/jschmidt/totally-speedy-transmute/safe/src/lib.rs /Users/jschmidt/totally-speedy-transmute/src/lib.rs /Users/jschmidt/totally-speedy-transmute/tests/wat.rs`
Processing file /Users/jschmidt/totally-speedy-transmute/safe/src/lib.rs
Processing file /Users/jschmidt/totally-speedy-transmute/src/lib.rs
Processing file /Users/jschmidt/totally-speedy-transmute/tests/wat.rs
Unsafe functions: 0/10
Unsafe expressions: 0/42
Unsafe traits: 0/0
Unsafe methods: 0/0
Unsafe impls: 0/0

$ poopd
fish: Unknown command: poopd

$ popd
```
