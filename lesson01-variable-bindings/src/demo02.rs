fn main() {
    let (x, y) = (1, 2);
    println!("x = {}, y = {},are they equal? {}", x, y,x == y);//x = 1, y = 2,are they equal? false
    // assert_eq!(x, y);
    // thread 'main' panicked at 'assertion failed: `(left == right)`
    // left: `1`,
    // right: `2`', demo02.rs:4:5
    // note: Run with `RUST_BACKTRACE=1` for a backtrace.
}