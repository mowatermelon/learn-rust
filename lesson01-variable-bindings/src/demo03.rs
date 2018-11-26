fn main() {
    let (a, mut b): (bool, bool) = (true, false); //声明一个静态变量a，和一个动态变量b
    println!("1 a = {}, b = {},are they equal? {}", a, b, a == b); //1 a = true, b = false,are they equal? false

    //静态变量不能进行再次修改，会直接报错
    // a = false;
    // error[E0384]: cannot assign twice to immutable variable `a`
    // --> demo02.rs:11:5
    // |
    // |     let (a, mut b): (bool,bool) = (true, false);//声明一个静态变量a，和一个动态变量b
    // |          - first assignment to `a`
    // ...
    // |     a = false;
    // |     ^^^^^^^^^ cannot assign twice to immutable variable
    // assert_eq!(a, b);

    //动态变量可以再次修改，会报错
    b = true;
    println!("2 a = {}, b = {},are they equal? {}", a, b, a == b); //2 a = true, b = true,are they equal? true
    assert_eq!(a, b);

    //将动态变量设置为静态变量
    let b = b;
    // warning: value assigned to `b` is never read
    // --> demo03.rs:23:9
    // |
    // |     let b = b;
    // |         ^
    // |
    // = note: #[warn(unused_assignments)] on by default

    //不能赋值
    // b = false;
    // println!("3 a = {}, b = {},are they equal? {}", a, b, a == b); //2 a = true, b = true,are they equal? true
    // assert_eq!(a, b);
    // error[E0384]: cannot assign twice to immutable variable `b`
    // --> demo03.rs:26:5
    // |
    // 23 |     let b = b;
    // |         - first assignment to `b`
    // ...
    // 26 |     b = false;
    // |     ^^^^^^^^^ cannot assign twice to immutable variable

    // error: aborting due to previous error

    // For more information about this error, try `rustc --explain E0384`.

    let mut b1 = 0b1111_0000; //有符号二进制
    println!("1 b1 is {}", b1); //1 b1 is 240
    b1 = 0o7320_1546; //有符号八进制

    //     warning: literal out of range for u8
    // --> demo03.rs:51:10
    // |
    // |     b1 = 0o7320_1546; //有符号八进制
    // |          ^^^^^^^^^^^
    // |
    // = note: #[warn(overflowing_literals)] on by default
    println!("2 b1 is {}", b1); //2 b1 is 102

    b1 = b1 as u8; //无符号八进制

    //     warning: literal out of range for u8
    //   --> demo03.rs:51:10
    //    |
    //    |     b1 = 0o7320_1546; //有符号八进制
    //    |          ^^^^^^^^^^^
    //    |
    //    = note: #[warn(overflowing_literals)] on by default
    println!("3 b1 is {}", b1); //3 b1 is 102

    let mut b1 = 0xf23a_b049; //有符号十六进制

    // warning: literal out of range for u16
    // --> demo03.rs:57:18
    // |
    // |     let mut b1 = 0xf23a_b049; //有符号十六进制
    // |                  ^^^^^^^^^^^
    // |
    // = note: the literal `0xf23a_b049` (decimal `4063932489`) does not fit into an `u16` and will become `45129u16`
    // = help: consider using `u32` instead

    println!("4 b1 is {}", b1); //4 b1 is 45129

    b1 = b1 as u16; //无符号八进制
    println!("5 b1 is {}", b1); //5 b1 is 45129

    // 无符号整数转有符号整数
    // 1u8 ---> 1i8
    // 1u16 ---> 1i16
    // 1u32 ---> 1i32
    // 1u64 ---> 1i64
    // 1usize ---> 1isize

    // 浮点类型内部转化
    // 1f32 ---> 1f64
}
