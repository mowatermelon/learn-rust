fn main() {
    let x: i32 = 8;
    {
        println!("1 {}", x); //1 8
        let x = 12;
        println!("2 {}", x); //2 12
    }
    println!("3 {}", x); // 3 8
    let x = 42;
    println!("4 {}", x); // 4 42

    let y = 4;
    println!("1 {}", y); // 1 4

    let y = "I can also be bound to text!"; // y现在更换了一个数据类型
    println!("2 {}", y); // 2 I can also be bound to text!

}
