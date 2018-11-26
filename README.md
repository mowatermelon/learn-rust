# learn_rust ---begin

## To Use

### check environment

To clone and run this repository you'll need [Git](https://git-scm.com), [Node.js](https://nodejs.org/en/download/) (which comes with npm) and [rustc](https://win.rustup.rs/) installed on your computer. From your command line.

If you don't know how to install `rustc`, can refer to [run foundation project of Rust study](https://coding.net/u/melonHero/p/learn-rust/wiki/1)。

```sh
$ git --version
git version 2.10.2.windows.1

$ node -v
v8.10.0

$ npm -v
5.8.0

$ rustc --version
rustc 1.27.2 (58cc626de 2018-07-18)

$ cargo --version
cargo 1.27.0 (1e95190e5 2018-05-27)
```

### clone project

```sh
# clone this repository
$ git clone -b begin https://git.coding.net/melonHero/learn-rust.git begin

# go into the repository
cd begin
```

### run project

```sh
# enter the project to execute the command
$ cargo run
# or
$ rustc src/xxx.rs
```

### build project

```sh
# enter the project to execute the command
cargo build
```

### get help

```sh
# enter the project to execute the command
cargo --help
# Alternatively, look at the local built-in documentation for rustup
rustup doc
```

## project list

- lesson01-variable-bindings

## other language version description

- [中文简体版说明](./lang/zh-cn-simple.md)