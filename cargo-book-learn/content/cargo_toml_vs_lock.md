<!--
 * @Date: 2020-09-27 17:16:24
 * @LastEditTime: 2020-09-27 17:44:07
-->

# Cargo.toml vs Cargo.lock

`Cargo.toml` and `Cargo.lock` serve two different purposes. Before we talk about them, here’s a summary:

- `Cargo.toml` is about describing your dependencies in a broad sense, and is written by you.
- `Cargo.lock` contains exact information about your dependencies. It is maintained by Cargo and should not be manually edited.

If you’re building a non-end product, such as a rust library that other rust packages will depend on, put `Cargo.lock` in your `.gitignore`. If you’re building an end product, which are executable like command-line tool or an application, or a system library with crate-type of `staticlib` or `cdylib`, check `Cargo.lock` into `git`.

Let’s dig in a little bit more.
Cargo.toml is a manifest file in which we can specify a bunch of different metadata about our package. For example, we can say that we depend on another package:

``` rust
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
```

This package has a single dependency, on the rand library. We’ve stated in this case that we’re relying on a particular Git repository that lives on GitHub. **Since we haven’t specified any other information, Cargo assumes that we intend to use the latest commit on the master branch to build our package.**

> Sound good? Well, there’s one problem: If you build this package today, and then you send a copy to me, and I build this package tomorrow, something bad could happen. There could be more commits to rand in the meantime, and my build would include new commits while yours would not. Therefore, we would get different builds. This would be bad because we want reproducible builds.

We could fix this problem by putting a rev line in our Cargo.toml:

``` rust
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git", rev = "9f35b8e" }
```

Now our builds will be the same. But there’s a big drawback: now we have to manually think about SHA-1s every time we want to update our library. This is both tedious and error prone.
每次更新库都要考虑sha-1s，很麻烦

Enter the Cargo.lock. Because of its existence, we don’t need to manually keep track of the exact revisions: Cargo will do it for us. When we have a manifest like this:

``` rust
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
```

Cargo will take the latest commit and write that information out into our Cargo.lock when we build for the first time. That file will look like this:

``` rust
[[package]]
name = "hello_world"
version = "0.1.0"
dependencies = [
 "rand 0.1.0 (git+https://github.com/rust-lang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9)",
]

[[package]]
name = "rand"
version = "0.1.0"
source = "git+https://github.com/rust-lang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9"
```

You can see that there’s a lot more information here, including the exact revision we used to build. Now when you give your package to someone else, they’ll use the exact same SHA, even though we didn’t specify it in our Cargo.toml.

When we’re ready to opt in to a new version of the library, Cargo can re-calculate the dependencies and update things for us:

``` bash
 cargo update           # updates all dependencies
 cargo update -p rand   # updates just “rand”
```

This will write out a new Cargo.lock with the new version information. Note that the argument to cargo update is actually a Package ID Specification and rand is just a short specification.