fn main() {
    let x: i32;
    // warning: unused variable: `x`
    // --> demo05.rs:2:9
    // |
    // |     let x: i32;
    // |         ^ help: consider using `_x` instead
    // |
    // = note: #[warn(unused_variables)] on by default
    println!("Hello watermelon!");//Hello watermelon!

    println!("Hello watermelon! {}",x);//Hello watermelon!
    // error[E0381]: use of possibly uninitialized variable: `x`
    // --> demo05.rs:12:37
    // |
    // 12 |     println!("Hello watermelon! {}",x);//Hello watermelon!
    // |                                     ^ use of possibly uninitialized `x`

    // error: aborting due to previous error

    // For more information about this error, try `rustc --explain E0381`.
}