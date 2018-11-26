// if not use ,it will failed to resolve. and see message,use of undeclared type or module `fmt`
use std::fmt; // Import `fmt`

// is a structure which contains two `i64`
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.so that MinMax can use normal print {}
// if not inheritance , `MinMax` cannot be formatted with the default formatter
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Implement `Binary` for `MinMax`.
impl fmt::Binary for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "Binary ({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement for Point2D
// so that Point2D can use normal print {}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:"); //Compare structures:
    println!("Display: {}", minmax); //Display: (0, 14)
    println!("What does minmax look like in binary: {:b}?", minmax);//What does minmax look like in binary: Binary (0, 14)?

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    //if the trait `std::fmt::Display` is not implemented for `MinMax`，
    //small can not equal small_range,
    //big can not equal big_range,
    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    ); //The big range is (-300, 300) and the small is (-3, 3)

    let point = Point2D { x: 3.3, y: 7.2 }; //The big range is (-300, 300) and the small is (-3, 3)

    println!("Compare points:"); //Compare points:
    println!("Display: {}", point); //Display: x: 3.3, y: 7.2

    // println!("What does Point2D look like in binary: {:b}?", point);
    //     error[E0277]: the trait bound `Point2D: std::fmt::Binary` is not satisfied
    //   --> example05-display.rs:53:62
    //    |
    // 63 |     println!("What does Point2D look like in binary: {:b}?", point);
    //    |                                                              ^^^^^ the trait `std::fmt::Binary` is not implemented for `Point2D`
    //    |
    //    = note: required by `std::fmt::Binary::fmt`
}
