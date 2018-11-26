// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);//12 months in a year.
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");//"Christian" "Slater" is the "actor\'s" name.

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));//Now Structure(3) will print!
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));//Now Deep(Structure(7)) will print!
}
