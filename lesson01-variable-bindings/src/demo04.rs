fn main() {
    // boolean type
    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);//true false

    // char type
    let c = 'c';
    println!("{}", c);//c

    // numeric types
    let x = 42;
    let y: u32 = 123_456;
    let z: f64 = 1.23e+2;
    let zero = z.abs();
    println!("{} {} {} {}", x, y, z, zero);//42 123456 123 123

    let bin = 0b1111_0000;
    let oct = 0o7320_1546;
    let hex:u32 = 0xf23a_b049;
    println!("{} {} {}", bin, oct, hex);//240 15532902 4063932489

    // string types
    let str = "Hello, world!";
    let string1 = &str;
    let string2 = str.to_string();
    println!("{} {} {}", str, string1,string2);//Hello, world! Hello, world! Hello, world!

    // arrays and slices
    let a = [0, 1, 2, 3, 4];
    let ten_zeros: [i64; 10] = [0; 10];
    println!("{:?} {:?}", a, ten_zeros);//[0, 1, 2, 3, 4] [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    // slices
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4]; //slicing needs to be based on array mode
    println!("{:?} {:?}", a, middle);//[0, 1, 2, 3, 4] [1, 2, 3]

    // tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;
    println!("{:?} {} {}", tuple, fifty, hello);//(50, "hello") 50 hello

    // raw pointers
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };
    println!("{} {:?} {}", x, raw, points_at);//5 0xbddd10f6bc 5

    // functions
    fn foo(x: i32) -> i32 {
        x
    }
    let _bar: fn(i32) -> i32 = foo;
    // println!("{:?} {:?}", foo, _bar);
    //     error[E0277]: `fn(i32) -> i32 {main::foo}` doesn't implement `std::fmt::Debug`
    // --> demo04.rs:54:27
    // |
    // |     println!("{:?} {:?}", foo, _bar);
    // |                           ^^^ `fn(i32) -> i32 {main::foo}` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
    // |
    // = help: the trait `std::fmt::Debug` is not implemented for `fn(i32) -> i32 {main::foo}`
    // = note: required by `std::fmt::Debug::fmt`

    // error: aborting due to previous error

    // For more information about this error, try `rustc --explain E0277`.
}
