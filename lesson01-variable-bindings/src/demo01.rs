fn main() {
    let a1 = 5;//let 绑定 整数变量默认类型推断是 i32
    let a2: i32 = 5;//也可以不使用默认类型，直接指定类型
    println!("a1 is {} ,a2 is {},are they equal? {}", a1, a2, a1 == a2);//a1 is 5 ,a2 is 5,are they equal? true
    assert_eq!(a1, a2);//断言如果不正确程序会报错直接退出

    let b1: u32 = 5;//也可以不使用默认类型，直接指定类型
    // 如果一个变量 没有被使用过，会出现警告，警告是不会阻塞程序编译
    // warning: unused variable: `b1`
    // |
    // |     let b1:u32 = 5;
    // |         ^^ help: consider using `_b1` instead
    // |
    //   = note: #[warn(unused_variables)] on by default
    // let _b1: u32 = 5; 但是这样声明就没有问题
    // println!("{} {} {}", a1, b1, a1 == b1);
    // error[E0308]: mismatched types
    // |
    // |     println!("a1 is {} ,b1 is {},are they equal? {}",a1, b1, a1 == b1);
    // |                                        ^^ expected i32, found u32
    // assert_eq!(a1, b1);
    // 去掉上面的注释会报错，因为类型不匹配
    // errer: mismatched types
    //    |
    //    |     assert_eq!(a1, b1);
    //    |     ^^^^^^^^^^^^^^^^^^^ expected i32, found u32
    //    |
    //    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

}
