# learn_rust ---web service

## To Use

### check environment

To clone and run this repository you'll need [Git](https://git-scm.com), [Node.js](https://nodejs.org/en/download/) (which comes with npm) and [rustc](https://win.rustup.rs/) installed on your computer. From your command line:

```sh
$ git --version
git version 2.10.2.windows.1

$ node -v
v8.11.1

$ npm -v
5.8.0

$ rustc --version
rustc 1.30.0-nightly (d41f21f11 2018-08-24)

$ cargo --version
cargo 1.29.0-nightly (6a7672ef5 2018-08-14)
```

## run project

```sh
# clone this repository
$ git clone -b webService https://git.coding.net/melonHero/learn-rust.git webService

# go into the repository
cd webService

# rocket always asks for the latest version of the nightly version Rust. If your Rocket application suddenly fails to compile. Make sure you're using the latest nightly Rust. If not using the following command to upgrade
rustup update && cargo update

# run this project
$ cargo run
  # you can see
  Compiling web_service v0.1.0 ({the root folder of the current project})
    Finished dev [unoptimized + debuginfo] target(s) in {your project compile time}s
     Running `target\debug\melon.exe`

    Finished dev [unoptimized + debuginfo] target(s) in 22.56s
     Running `target\debug\web_service.exe`
🔧  Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 8
    => secret key: generated
    => limits: forms = 32KiB
    => tls: disabled
🛰  Mounting '/':
    => GET /
    => GET /<file..>
👾  Catchers:
    => 404
    => 500
🚀  Rocket has launched from http://localhost:8000
# The service will start until,until you hit CTRL+C
```

## build project

```sh
cargo build
```

## get help

```sh
cargo --help
```