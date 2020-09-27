<!--
 * @Date: 2020-09-27 17:16:24
 * @LastEditTime: 2020-09-27 18:08:42
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

## 小结

### 直观区别

- toml是你写的依赖，取决于你。
- lock，取决于cargo，它会自动拉取依赖的依赖，程序员不能手动修改。

#### 潜在问题

如果我们dependencies指定了一个git仓库，而又没有其他信息，那么cargo构建的时候，会默认使用这个git master分支最后一次提交。
这会有潜在的问题，比如你构建了一次项目，我第二天构建项目，但是中间这个依赖的git又有几次提交，那这样，你我构建的项目肯定是不一样的。

也可通过在git后加上rev的号来指定是哪一个位置的构建，那这会引入另一个问题，每次更新我们都要制定这个号，比较繁琐麻烦。

#### lock出现

lock解决了不一致的问题，每一次构建的时候，lock会记录额外的信息，所以如果这时候你给别人包的时候，如果有lock文件，那么我们构建的肯定是一样的，因为lock自动记录了额外的版本信息，即使我们不在toml中声明也可以。

#### 更新

当依赖的库需要更新，我们使用命令来完成，而且cargo会重新计算那些额外信息生成新的lock文件，更新全部的和特定的依赖都支持。

``` bash
cargo update           # updates all dependencies
cargo update -p rand   # updates just “rand”
```

#### 其他

如果你构建的不是最终产品，那么可以把lock文件加入.gitignore, 这时候你的其他依赖都是不确定的，所以无需上传lock。
如果你构建的是最终产品，那么可以把lock放入仓库，因为这时候你的产品是完整的，依赖是确定的。
