# 学习rust ---完整版

## 如何使用

### 先确认环境

需要你电脑中已经安装[Git](https://git-scm.com)，[Node.js](https://nodejs.org/en/download/) (一般安装node都会内置npm)和 [rustc](https://win.rustup.rs/) ，你可以再你电脑终端中输入下面指令，检测你本地环境是否安装完成。

如果不知道如何安装`rustc`，可以参考[学习Rust之运行基础项目](https://coding.net/u/melonHero/p/learn-rust/wiki/1)。

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

### 克隆项目

```sh
# 克隆这个存储库
$ git clone -b begin https://git.coding.net/melonHero/learn-rust.git begin

# 进入本地仓库文件夹
cd begin
```

### 如何运行项目

```sh
# 进入对应项目文件夹，并且执行以下指令
$ cargo run
# or
$ rustc src/xxx.rs
```

### 如何构建项目

```sh
# 进入对应项目文件夹，并且执行以下指令
cargo build
```

### 寻求帮助

```sh
# 进入对应项目文件夹，并且执行以下指令
cargo --help
# 或者，查看rustup本地内置文档
rustup doc
```

## 项目学习参考列表

- 课程一  变量绑定与原始类型

## 其他语言描述版本

- [English version](../README.md)