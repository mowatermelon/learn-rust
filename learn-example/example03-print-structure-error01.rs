//the trait `std::fmt::Debug` is not implemented for `Structure`
// so `Structure` and it's children cannot be formatted using `{:?}`

struct Structure(i32);
struct Deep(Structure);
fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    ); //"Christian" "Slater" is the "actor\'s" name.

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    // error[E0277]: `Structure` doesn't implement `std::fmt::Debug`
    //   --> example03-debug-error.rs:12:38
    //    |
    // 12 |     println!("Now {:?} will print!", Structure(3));
    //    |                                      ^^^^^^^^^^^^ `Structure` cannot be formatted using `{:?}`
    //    |
    //    = help: the trait `std::fmt::Debug` is not implemented for `Structure`
    //    = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
    //    = note: required by `std::fmt::Debug::fmt`

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
    // error[E0277]: `Deep` doesn't implement `std::fmt::Debug`
    // --> example03-debug-error.rs:16:38
    // |
    // 16 |     println!("Now {:?} will print!", Deep(Structure(7)));
    // |                                      ^^^^^^^^^^^^^^^^^^ `Deep` cannot be formatted using `{:?}`
    // |
    // = help: the trait `std::fmt::Debug` is not implemented for `Deep`
    // = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
    // = note: required by `std::fmt::Debug::fmt`
}
