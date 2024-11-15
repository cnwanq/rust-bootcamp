# RUST 项目结构

许多小伙伴在学习 Rust 的时候会比较困惑，就是我写的项目文件结构到底对不对，是不是标准的？下面我们就由点及面，看一看一个大的 Rust 项目是如何组织代码的，我们从最基本的 main.rs 以及 lib.rs 开始

## create

- Crate 是 Rust 的基本编译单元。每个 crate 都是一个独立的编译目标，可以是一个库（lib crate）或一个可执行文件（binary crate）。
- 一个 crate 有一个根文件，对于库 crate 是 src/lib.rs，对于二进制 crate 是 src/main.rs。

## Package

但是一个最基础的 rust 项目，只有这两个文件肯定是不够的，
Package 是一个包含一个或多个 crate 的集合，包含 Cargo.toml 和 Cargo.lock 文件，用于定义包的元数据和依赖关系。
在实际项目中，crate 只包含代码和模块，而 Cargo.toml 和 Cargo.lock 文件是 package 的一部分，负责管理和构建整个包。
例如我们使用 cargo new sdk 创建一个库的时候，里面会有以下文件

### 文件结构示例

```bash
//库文件
sdk/
├── Cargo.toml
├── Cargo.lock
└── src
    └── lib.rs

or

//可执行文件
sdk/
├── Cargo.toml
├── Cargo.lock
└── src
    └── main.rs
```

### toml 文件

toml 文件用来管理依赖和版本信息，例如：

```toml
[package]
name = "sdk"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

添加一个比较常见用来简化错误处理的包：thiserror,可以使用 cargo add thiserror 命令,

```toml
[package]
name = "sdk"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
thiserror = "1.0.61"
```

### 测试代码以及性能测试

到这里我们已经是在写一个完整的项目了，但是一个项目必不可少的就是各种测试，单元测试，性能测试，那这些文件，应该放在哪里呢？我们继续拿这个 sdk 项目进行说明。

在官方以及大社区大家所遵守的标准是，在 src 同级目录下创建 tests 以及 benches 文件夹，如下

```bash
  sdk/
  ├── Cargo.toml
  ├── src/
  │   └── lib.rs
  ├── tests/
  │   ├── some-integration-tests.rs
  │   └── multi-file-test/
  │       ├── main.rs
  │       └── test_module.rs
  └── benches/
      ├── large-input.rs
      └── multi-file-bench/
          ├── main.rs
          └── bench_module.rs

```

开始在编写项目的时候，可以讲单元测试直接放在所编辑文件的下面，这样就不需要创建 multi-file-test 及以下的文件了，但是随着开发的前进，测试代码可能开始逐渐占据比较大的篇幅的时候，为了让代码保持清洁，还是建议将其放到 tests 文件夹下。其中

- tests 存放的是功能测试代码，主要测试功能是否实现
- benches 存放的是性能测试代码，主要测试性能如何（例如一些服务接口性能测试）

## Workspace

如果我一个大的项目要由多个 rust 项目组成呢？例如面 sdk 是一个人负责编写的 rust 项目，基于此我们还需要构建一个 cli 项目和一个 server 项目，这个时候应该怎么办呢？分割代码到 3 个项目吗？这样看起来好像乱乱的，有没有统一管理的方式呢？有的，请看 Workspace

在 Rust 中，工作空间（workspace）是一种组织和管理多个包的方式，允许你在一个项目中包含多个相互关联的包。工作空间提供了一些工具和机制，使得管理多个包的依赖、构建和测试更加简单和一致。

### 工作空间的作用

1. 组织多个包：工作空间允许你将多个包组合在一起，这些包可以是库（lib crate）、命令行工具（CLI crate）或其他类型的包。
2. 共享依赖：工作空间中的所有包共享一个 Cargo.lock 文件，确保所有包使用相同版本的依赖，避免依赖版本冲突。
3. 简化构建流程：你可以在工作空间的根目录中运行 cargo build、cargo test 等命令，这些命令会递归地构建、测试工作空间中的所有包。
4. 一致性：通过共享的 Cargo.lock 文件和统一的构建命令，工作空间确保所有包的一致性和协调性。

### 工作空间的结构

我们继续假设，我们要将 sdk , cli , server 放到同一个工作空间，样子是这样的

一个典型的工作空间包含一个顶层目录，该目录包含一个根 Cargo.toml 文件和多个子包，每个子包有自己的 Cargo.toml 文件和源代码目录。

```bash
my_workspace /
├── Cargo.lock
├── Cargo.toml
├── crates/
│   ├── sdk/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └──── tests/
│   │       ├── some-integration-tests.rs
│   │       └── multi-file-test/
│   │           ├── main.rs
│   │           └── test_module.rs
│   ├── cli/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── main.rs
│   │   ├── bin/
│   │   │   ├── named-executable.rs
│   │   │   ├── another-executable.rs
│   │   │   └── multi-file-executable/
│   │   │       ├── main.rs
│   │   │       └── some_module.rs
│   │   └──── tests/
│   │       ├── some-integration-tests.rs
│   │       └── multi-file-test/
│   │           ├── main.rs
│   │           └── test_module.rs
│   └── server/
│       ├── Cargo.toml
│       ├── src/
│       │   └── main.rs
│       ├── bin/
│       │   ├── named-executable.rs
│       │   ├── another-executable.rs
│       │   └── multi-file-executable/
│       │       ├── main.rs
│       │       └── some_module.rs
│       ├── tests/
│       │   ├── some-integration-tests.rs
│       │   └── multi-file-test/
│       │       ├── main.rs
│       │       └── test_module.rs
│       └── benches/
│           ├── large-input.rs
│           └── multi-file-bench/
│               ├── main.rs
│               └── bench_module.rs
```

### workspace toml 文件

- 指定解析器，resolver = "2" 指定工作空间使用 Cargo 的第二版解析器。更好用

```toml
[workspace]
resolver = "2"
```

- 填写 workspace 信息

```toml
[workspace.package]
name = "my-workspace"
version = "0.1.0"
edition = "2021"
```

- 添加成员包，这样在编译的时候，编译程序就可以通过 members 进行编译了

```toml
[workspace]
members = [
    "crates/sdk",
    "crates/cli",
    "crates/server",
]
```

- 添加成员包需要的依赖

```toml
[workspace.dependencies]
thiserror = "1.0.61"
```

这一步大家可能会有疑惑，哎，我在 sdk 里面的 toml 里面不是有定义这一行代码吗？为什么要在这里再写一次呢？，这就不得不提到 workspace 的另一个功能了，管理项目依赖。

假如，我不仅在 sdk 项目中用到了 thiserror 库，我在 cli 项目以及 server 中也想用，我该怎么办呢？

可不可以～分别在对应项目中分别添加一次 thiserror = "1.0.61"呢？可以，当然可以，如果一开始没有 workspace,三个项目各写各的，那就是这么做，但是有一天我们想吧代码组织起来，放到一块，还能这么写吗？

大家思考一下，万一，三个项目使用的 thiserror 依赖不同的版本怎么办？编译代码的时候是不是要编译三次？编译后的文件大小是不是也包含三个 thiserror?
所以为了缩短编译时间以及编译后的文件大小，我们可以在 workspace 中设置一个统一的版本，然后在各自的项目中修改依赖

```toml
# workspace 文件中
[workspace.dependencies]
thiserror = "1.0.61"
```

```toml
#  cli 以及 server 的 toml 文件中
[dependencies]
thiserror.workspace = true
```

- 项目间互相引用

为了在 cli 以及 server 中也能够使用 sdk 中的方法，我们需要将我们的成员包也添加到依赖当中

```toml
[workspace.dependencies]
thiserror = "1.0.61"

# workspace members

cli = { path = "crates/cli" }
server = { path = "crates/server" }
sdk = { path = "crates/sdk" }
```

然后在项目包中添加该依赖，就可以在项目中使用了。

```toml
thiserror.workspace = true
sdk.workspace = true
```
