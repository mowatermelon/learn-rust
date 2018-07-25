# learn_rust ---hello world

## To Use

### check environment

To clone and run this repository you'll need [Git](https://git-scm.com), [Node.js](https://nodejs.org/en/download/) (which comes with npm) and [rustc](https://win.rustup.rs/) installed on your computer. From your command line:

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

## run project

```sh
# clone this repository
$ git clone -b bird https://git.coding.net/melonHero/learn_rust.git bird

# go into the repository
cd bird

# run this project
$ cargo run
  # you can see
   Compiling melon v0.2.0 ({the root folder of the current project})
    Finished dev [unoptimized + debuginfo] target(s) in {your project compile time}s
     Running `target\debug\melon.exe`

---------------------Guess the number!

Please input your guess:
20
The number you guessed is too small!

Please input your guess:
53
The number you guessed is too big!

# until you enter the correct number

Please input your guess:
43
You win!

---------------------Yes,the secret number is: 43


---------------------Guess the number!

Please input your guess:

# You can always start playing again and guess at the game,until you hit CTRL+C
```

## build project

```sh
cargo build
```

## get help

```sh
cargo --help
```