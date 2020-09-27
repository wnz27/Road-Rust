<!--
 * @Date: 2020-09-27 16:03:28
 * @LastEditTime: 2020-09-27 17:12:43
-->

# Create a new package

`cargo new hello_world --bin`

We’re passing --bin because we’re making a binary program: if we were making a library, we’d pass --lib. This also initializes a new git repository by default. If you don't want it to do that, pass --vcs none

You’ll now notice a new file, Cargo.lock. It contains information about our dependencies. Since we don’t have any yet, it’s not very interesting.

Once you’re ready for release, you can use `cargo build --release` to compile your files with optimizations turned on:

```
$ cargo build --release
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
```

`cargo build --release` puts the resulting binary in **target/release** instead of **target/debug**.
Compiling in debug mode is the default for development.
Compilation time is shorter since the compiler doesn't do optimizations, but the code will run slower.(default)
Release mode takes longer to compile, but the code will run faster.(realease)

# Working on an Existing Cargo Package
If you download an existing package that uses Cargo, it’s really easy to get going.
First, get the package from somewhere. In this example, we’ll use rand cloned from its repository on GitHub:
```
$ git clone https://github.com/rust-lang-nursery/rand.git
$ cd rand
```
To build, use cargo build:
```
$ cargo build
   Compiling rand v0.1.0 (file:///path/to/package/rand)
```
This will fetch all of the dependencies and then build them, along with the package.

# Package Layout
Cargo uses conventions for file placement to make it easy to dive into a new Cargo package:

```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```
Cargo.toml and Cargo.lock are stored in the root of your package (package root).
Source code goes in the src directory.
The default library file is src/lib.rs.
The default executable file is src/main.rs.
Other executables can be placed in src/bin/.
Benchmarks go in the benches directory.
Examples go in the examples directory.
Integration tests go in the tests directory.

[【返回上一级】](../cargo-category.md)
