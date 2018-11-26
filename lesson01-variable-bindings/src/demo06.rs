fn main() {
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); // This won't work.
    // error[E0425]: cannot find value `y` in this scope
    // --> demo06.rs:7:62
    // |
    // |     println!("The value of x is {} and value of y is {}", x, y); // This won't work.
    // |                                                              ^ did you mean `x`?

    // error: aborting due to previous error

    // For more information about this error, try `rustc --explain E0425`.
}