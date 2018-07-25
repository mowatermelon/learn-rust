# 一、安装环境

> 搬运官网

## 1 使用 `rustup` 管理工具链

`Rust` 由 `rustup` 工具来安装和管理。 `Rust` 有一个 6 周的 快速发布过程 并且支持 大量的平台 ，所以任何时候都有很多 Rust 构建可用。 `rustup` 在 `Rust` 支持的每一个平台上以一致的方式管理这些构建， 并可以从 `beta` 和 `nightly` 发布渠道安装 `Rust`，且支持额外的交叉编译目标平台。

更多信息请查看 [rustup documentation](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md)。

## 2 配置 PATH 环境变量

在 `Rust` 开发环境中，所有工具都安装到 `%USERPROFILE%\.cargo\bin` 目录， 并且您能够在这里找到 `Rust` 工具链，包括 `rustc`、`cargo` 及 `rustup`。

因此，`Rust` 开发者们通常会将此目录放入 `PATH` 环境变量。在安装时，`rustup` 会尝试配置 `PATH`， 但是因为不同平台、命令行之间的差异，以及 `rustup` 的 `bug`，对于 `PATH` 的修改将会在重启终端、用户登出之后生效，或者有可能完全不会生效。

当安装完成之后，如果在控制台运行 `rustc --version` 失败，这是最可能的原因。

## 3 Windows 注意事项

在 `Windows` 上， `Rust` 需要 `Visual C++` 生成工具 `2013` 或更新版本的支持。获取 `Visual C++` 生成工具最方便的方法是安装 `Microsoft Visual C++ Build Tools 2017` 。或者，你可以 安装 `Visual Studio 2017`, `Visual Studio 2015`, 或 `Visual Studio 2013` 并在安装过程中选择安装 `C++ 工具`。

关于在 `Windows` 上配置 `Rust` 的更多信息请查看 [Windows-specific rustup](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-rust-on-windows) 文档。

# 二、确认安装是否成功

安装`Rust`之后，可以使用以下命令查看`rustc`和`cargo`的版本信息。

`cargo`是`Rust`内置的项目管理工具。用于`Rust` 项目的创建、编译、运行，同时对项目的依赖进行管理，自动判断使用的第三方依赖库，进行下载和版本升级。

```sh
$ rustc --version
rustc 1.27.2 (58cc626de 2018-07-18)

$ cargo --version
cargo 1.27.0 (1e95190e5 2018-05-27)

# modify installation version
rustup install nightly
#or
rustup install beta

# view local documents
rustup doc
```

# 三、更换镜像源

由于`Cargo` 使用的是亚马逊在美国的服务器，所以默认先更换成[中科大的镜像源](https://lug.ustc.edu.cn/wiki/mirrors/help/rust-crates)

```sh
# 修改对应config文件，win系统和mac系统，都可以执行这条指令
$ vi $HOME/.cargo/config

# 打开config之后，里面应该是一个空文件，将以下内容输入进去
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"


# 注意，如果所处的环境中不允许使用 git 协议, 可以把上述最后一行改为
registry = "http://mirrors.ustc.edu.cn/crates.io-index"
```

# 四、创建新项目

## 1、使用 `new 项目名称` 创建新项目

```sh
# 创建`library`项目
cargo new hello_world

# 创建可运行的 `Rust` 项目
cargo new hello_world --bin
```

## 2、项目结构

```text
├── src                                         // 项目代码目录
│   ├── main.rs                              // 项目启动文件
├── Cargo.toml                                  // cargo配置文件
```

### Cargo.toml

```toml
[package]

name = "{your project name}"
version = "{your project version}"
authors = ["{your name} <{your email}>"]

# eg

[package]
name = "melon"
version = "0.1.0" # the default version is 0.1.0
authors = ["watermelon <neefoxmo@gmail.com>"] # this will automatically read your local git user name and email configuration

[dependencies]
```

### main.rs

```rust
fn main() {
    println!("Hello, world!");
}
// you can modify it
fn main() {
    println!("Hello, watermelon!");
}
```

# 五、编译项目

```sh
$ cargo build
   Compiling {your project name} v{your project version} ({the root folder of the current project})
    Finished dev [unoptimized + debuginfo] target(s) in {your project compile time}s

# eg my project name is melon
$ cargo build
   Compiling melon v0.1.0 (file:///E:/7_25/bak/learn_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 8.60s
# it automatically generates a target folder
```

> target 文件夹内结构

```text
├── debug
│   ├── .fingerprint
│   │   ├── melon-6b77381c902e8ce0
│   │   │   ├── bin-melon-6b77381c902e8ce0
│   │   │   ├── bin-melon-6b77381c902e8ce0.json
│   │   │   ├── dep-bin-melon-6b77381c902e8ce0
│   ├── build
│   ├── deps
│   │   ├── melon-6b77381c902e8ce0.d
│   │   ├── melon-6b77381c902e8ce0.exe
│   │   ├── melon-6b77381c902e8ce0.pdb
│   ├── examples
│   ├── incremental
│   │   ├── melon-3ljp75ifjy8x0
│   │   │   ├── s-f3816zo0kr-adkdgo-okm4wlmyhmvy
│   │   │   │   ├── 1y16o1qfye96o7m0.o
│   │   │   │   ├── 3rngp6bm2u2q5z0y.o
│   │   │   │   ├── 4oc10dk278mpk1vy.o
│   │   │   │   ├── 4xq48u46a1pwiqn7.o
│   │   │   │   ├── 5aakxp693n9z52lh.o
│   │   │   │   ├── 10ygozpgsu0ud2lu.o
│   │   │   │   ├── dep-graph.bin
│   │   │   │   ├── query-cache.bin
│   │   │   │   ├── work-products.bin
│   │   │   ├── s-f3816zo0kr-adkdgo.lock
│   ├── native
│   ├── .cargo-lock
│   ├── melon.d
│   ├── melon.exe
│   ├── melon.pdb
├── .rustc_info.json
```

# 六、运行项目

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in {your project run time}s
     Running `target\debug\{your project name}.exe`
Hello, world!

# eg my project name is melon
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target\debug\melon.exe`
Hello, watermelon!
```

# 七、获取帮助

## 1 获取所有帮助

```sh
$ cargo --help

Rust's package manager

USAGE:
    cargo.exe [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -V, --version           Print version info and exit
        --list              List installed commands
        --explain <CODE>    Run `rustc --explain CODE`
    -v, --verbose           Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet             No output printed to stdout
        --color <WHEN>      Coloring: auto, always, never
        --frozen            Require Cargo.lock and cache are up to date
        --locked            Require Cargo.lock is up to date
    -Z <FLAG>...            Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    -h, --help              Prints help information

Some common cargo commands are (see all commands with --list):
    build       Compile the current project
    check       Analyze the current project and report errors, but don't build object files
    clean       Remove the target directory
    doc         Build this project's and its dependencies' documentation
    new         Create a new cargo project
    init        Create a new cargo project in an existing directory
    run         Build and execute src/main.rs
    test        Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this project to the registry
    install     Install a Rust binary
    uninstall   Uninstall a Rust binary

See 'cargo help <command>' for more information on a specific command.
```

## 2 获取详细功能帮助

```sh
$ cargo {someword} --help

# eg
$ cargo new --help
cargo.exe-new
Create a new cargo package at <path>

USAGE:
    cargo.exe new [OPTIONS] <path>

OPTIONS:
        --vcs <VCS>       Initialize a new repository for the given version control system (git, hg, pijul, or fossil)
                          or do not initialize any version control at all (none), overriding a global configuration.
                          [possible values: git, hg, pijul, fossil, none]
        --bin             Use a binary (application) template [default]
        --lib             Use a library template
        --name <NAME>     Set the resulting package name, defaults to the directory name
    -v, --verbose         Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet           No output printed to stdout
        --color <WHEN>    Coloring: auto, always, never
        --frozen          Require Cargo.lock and cache are up to date
        --locked          Require Cargo.lock is up to date
    -Z <FLAG>...          Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    -h, --help            Prints help information

ARGS:
    <path>
```
