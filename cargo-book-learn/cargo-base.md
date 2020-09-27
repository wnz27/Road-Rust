<!--
 * @Date: 2020-09-27 12:52:56
 * @LastEditTime: 2020-09-27 16:36:57
-->

# Introduction

install:
On Linux and macOS systems, this is done as follows:
`$ curl https://sh.rustup.rs -sSf | sh`

use:

- create a new project: `cargo new project-name`

>This is all we need to get started. First, let’s check out Cargo.toml:

```[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

> This is called a manifest, and it contains all of the metadata that Cargo needs to compile your package.

- compile project: `cargo build`

> Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)

- run compiled project: `./target/debug/project-name`
- compile and run project: `cargo run`

> ps: you must cd to your project-name dir, or raise a error: could not find Cargo.toml in `xx/xx/xx` or any parent directory

> You won't see the Compiling line if you have not made any changes since you last compiled

Why Cargo Exists

Cargo is a tool that allows Rust packages to declare their various dependencies and
ensure that you’ll always get a repeatable build.

To accomplish this goal, Cargo does four things:

1. Introduces two metadata files with various bits of package information.
2. Fetches and builds your package’s dependencies.
3. Invokes `rustc` or another build tool with the correct parameters to build your package.
4. Introduces conventions to make working with Rust packages easier.(制定自己的规则，使用规范化，所以让打包变得简单)
