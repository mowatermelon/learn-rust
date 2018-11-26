// format!: write formatted text to String
// print!: same as format! but the text is printed to the console (io::stdout). some like console.log in js
// println!: same as print! but a newline is appended.
// eprint!: same as format! but the text is printed to the standard error (io::stderr). some like console.error in js
// eprintln!: sames as eprint!but a newline is appended.
fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    // format!("  format! {} days", 31);//  format! 31 days
    print!("  print! {} days", 31);//  print! 31 days
    println!("  println! {} days", 31);//  println! 31 days
    eprint!("  eprint! {} days", 31);//  eprint! 31 days
    eprintln!("  eprintln! {} days", 31);//  eprintln! 31 days

    // Without a suffix, 31 becomes an i32. You can change what type 31 is,
    // with a suffix.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    // format!("  format! {0}, this is {1}. {1}, this is {0}", "Alice", "Bob");//  format! Alice, this is Bob. Bob, this is Alice
    print!("  print! {0}, this is {1}. {1}, this is {0}", "Alice", "Bob");//  print! Alice, this is Bob. Bob, this is Alice
    println!("  println! {0}, this is {1}. {1}, this is {0}", "Alice", "Bob");//  println! Alice, this is Bob. Bob, this is Alice
    eprint!("  eprint! {0}, this is {1}. {1}, this is {0}", "Alice", "Bob");//  eprint! Alice, this is Bob. Bob, this is Alice
    eprintln!("  eprintln! {0}, this is {1}. {1}, this is {0}", "Alice", "Bob");//  eprintln! Alice, this is Bob. Bob, this is Alice
    
    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");//the quick brown fox jumps over the lazy dog

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);//1 of 10 people know binary, the other half doesn't

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);//     1

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);//000001

    // It will even check to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");
    // Run it. See? Now try deleting the two slashes, and run it again.
    // |                               ^^^
    // |
    // = note: positional arguments are zero-based

    // error: aborting due to previous error
    
    // Create a structure which contains an `i32`. Name it `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // Run it. See? Now try deleting the two slashes, and run it again.
    // |                                                 ^^^^^^^^^^^^ `main::Structure` cannot be formatted with the default formatter
    // |
    // = help: the trait `std::fmt::Display` is not implemented for `main::Structure`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // = note: required by `std::fmt::Display::fmt`

    //     error: aborting due to previous error

    //     For more information about this error, try `rustc --explain E0277`.
}


