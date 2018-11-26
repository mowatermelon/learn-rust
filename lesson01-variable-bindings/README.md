# 原生类型

`Rust`内置的原生类型 (`primitive types`) 有以下十类：

> 布尔类型（bool）

有两个值`true`和`false`，布尔型通常用在if语句中。

你可以在[标准库文档](https://doc.rust-lang.org/nightly/std/primitive.bool.html)中找到更多关于`bool`的文档。

```rust
let x = true;

let y: bool = false;
```

> 字符类型（char,单引号，直接字面量声明）

表示单个`Unicode`字符，存储为`4`个字节。

```rust
let x = 'x';
let two_hearts = '💕';
```

不像其它语言，这意味着`Rust`的`char`并不是 1 个字节，而是 4 个。

你可以在[标准库文档](https://doc.rust-lang.org/nightly/std/primitive.char.html)中找到更多关于`char`的文档。

> 数值类型（类型指定或者直接字面量声明）

分为有符号整数 (`i8`, `i16`, `i32`, `i64`, `isize`)、 无符号整数 (`u8`, `u16`, `u32`, `u64`, `usize`) 以及浮点数 (`f32`, `f64`)。

如果一个数字常量没有推断它类型的条件，它采用默认类型：

```rust
let x = 42; // `x` has type `i32`.

let y = 1.0; // `y` has type `f64`.
```

- 有符号和无符号

整型有两种变体：有符号和无符号。

为了理解它们的区别，让我们考虑一个 4 字节大小的数字。一个有符号，4 字节`-8`到`+7`的数字。有符号数采用`二进制补码`表示。一个无符号 4 字节的数字，因为它不需要储存负数，可以出储存`0`到`+15`的数字。

无符号类型使用`u`作为他们的分类，而有符号类型使用`i`。`i`代表`integer`。所以`u8`是一个 `8` 位无符号数字，而`i8`是一个 8 位有符号数字。

- 固定大小类型

固定大小类型在其表现中有特定数量的位。有效的位大小是`8`，`16`，`32`和`64`。那么，`u32`是无符号的，`32` 位整型，而`i64`是有符号，64 位整型。

- 可变大小类型

`Rust` 也提供了依赖底层机器指针大小的类型。这些类型拥有`size`分类，并有有符号和无符号变体。它有两个类型：`isize`和`usize`。

- 浮点类型

`Rust` 也有两个浮点类型：`f32`和`f64`。它们对应 `IEEE-754` 单精度和双精度浮点数。

这里有一个不同数字类型的列表，以及它们在标准库中的文档：

* [i8 -2^7~2^7-1](https://doc.rust-lang.org/nightly/std/primitive.i8.html)
* [i16 -2^15~2^15-1](https://doc.rust-lang.org/nightly/std/primitive.i16.html)
* [i32 -2^31~2^31-1](https://doc.rust-lang.org/nightly/std/primitive.i32.html)
* [i64 -2^63~2^63-1](https://doc.rust-lang.org/nightly/std/primitive.i64.html)
* [u8 ~2^8-1](https://doc.rust-lang.org/nightly/std/primitive.u8.html)
* [u16 0~2^16-1](https://doc.rust-lang.org/nightly/std/primitive.u16.html)
* [u32 0~2^32-1](https://doc.rust-lang.org/nightly/std/primitive.u32.html)
* [u64 0~2^64-1](https://doc.rust-lang.org/nightly/std/primitive.u64.html)
* [isize -2^63~2^63-1](https://doc.rust-lang.org/nightly/std/primitive.isize.html)
* [usize 0~2^64-1](https://doc.rust-lang.org/nightly/std/primitive.usize.html)
* [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)
* [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)


> 字符串类型（双引号，直接字面量声明）

最底层的是不定长类型`str`，更常用的是字符串切片`&str`和堆分配字符串`String`。

其中字符串切片是`静态分配`的，有固定的大小，并且不可变，而堆分配字符串是可变的。

> 数组（直接字面量声明）

具有固定大小，并且元素都是同种类型，可表示为`[T; N]`，`T`是数据类型，`N`是一个编译时常量，代表数组的长度。

有一个可以将数组中每一个元素初始化为相同值的简写。在这个例子中，a的每个元素都被初始化为0：

```rust
let mut array: [i32; 3] = [0; 3];

array[1] = 1;
array[2] = 2;

assert_eq!([1, 2], &array[1..]);

// This loop prints: 0 1 2
for x in &array {
    print!("{} ", x);
}
```

但是请注意，数组本身不可迭代：

```rust
let array: [i32; 3] = [0; 3];

for x in array { }
// error: the trait bound `[i32; 3]: std::iter::Iterator` is not satisfied

//解决方案是通过调用切片方法将数组强制转换为切片：
for x in array.iter() { }

//如果数组有32个或更少的元素（见上文），您还可以使用数组引用的IntoIterator实现：
for x in &array { }
```

你可以用`a.len()`来获取数组a的元素数量：

```rust
let a = [1, 2, 3];

println!("a has {} elements", a.len());
```

你可以用下标（subscript notation）来访问特定的元素：

```rust
let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]

println!("The second name is: {}", names[1]);
```

就跟大部分编程语言一个样，下标从`0`开始，所以第一个元素是`names[0]`，第二个是`names[1]`。上面的例子会打印出`The second name is: Brian`。如果你尝试使用一个不在数组中的下标，你会得到一个错误：数组访问会在运行时进行边界检查。这种不适当的访问是其它系统编程语言中很多`bug`的根源。
你可以在[标准库文档](http://doc.rust-lang.org/nightly/std/primitive.array.html)中找到更多关于`array`的文档。

> 切片（基于一个数组变量，直接字面量声明）

引用一个数组的部分数据并且不需要拷贝，可表示为`&[T]`。
一个切片`slice`是一个数组的引用（或者`视图`）。它有利于安全，有效的访问数组的一部分而不用进行拷贝。

比如，你可能只想要引用读入到内存的文件中的一行。原理上，片段并不是直接创建的，而是引用一个已经存在的变量。片段有预定义的长度，可以是可变也可以是不可变的。

在底层，`slice` 代表一个指向数据开始的指针和一个长度。

你可以用一个`&`和`[]`的组合从多种数据类型创建一个切片。&表明切片类似于引用，这个我们会在本部分的后面详细介绍。带有一个范围的`[]`，允许你定义切片的长度：

```rust
let a = [0, 1, 2, 3, 4];
let complete = &a[..]; // A slice containing all of the elements in `a`.
let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.

// slicing a Vec
let vec = vec![1, 2, 3];
let int_slice = &vec[..];
// coercing an array to a slice
let str_slice: &[&str] = &["one", "two", "three"];
```

切片是可变的或共享的。共享切片类型是`＆[T]`，而可变切片类型是`＆mut [T]`，其中`T`表示元素类型。例如，您可以改变可变切片指向的内存块：

```rust
let x = &mut [1, 2, 3];
x[1] = 7;
assert_eq!(x, &[1, 7, 3]);
```

方法

- len

`pub const fn len(&self) -> usize`
返回切片长度，返回值为`usize`

```rust
let a = [1, 2, 3];
assert_eq!(a.len(), 3);
```

- is_empty

`pub const fn is_empty(&self) -> bool`
如果切片的长度为`0`，则返回`true`。

```rust
let a = [1, 2, 3];
assert!(!a.is_empty());
```

- first

`pub fn first(&self) -> Option<&T>`
返回切片的第一个元素，如果为空，则返回`None`。

```rust
let v = [10, 40, 30];
assert_eq!(Some(&10), v.first());

let w: &[i32] = &[];
assert_eq!(None, w.first());
```

- first_mut

`pub fn first_mut(&mut self) -> Option<&mut T>`
返回指向切片的第一个元素的可变指针，如果为空，则返回`None`。

```rust
let x = &mut [0, 1, 2];

if let Some(first) = x.first_mut() {
    *first = 9;
}
assert_eq!(Some(&9), x.first());
assert_eq!(x, &[9, 1, 2]);
```

- split_first

`pub fn split_first(&self) -> Option<(&T, &[T])>`
返回切片的第一个和所有其余元素，如果为空，则返回`None`。

```rust
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first() {
    assert_eq!(first, &0);
    assert_eq!(elements, &[1, 2]);
}
```

- split_first_mut

`pub fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>`
返回切片的第一个指针和所有其余元素，如果为空，则返回`None`。

```rust
let x = &mut [0, 1, 2];

if let Some((first, elements)) = x.split_first_mut() {
    *first = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[3, 4, 5]);
```

- split_last

`pub fn split_last(&self) -> Option<(&T, &[T])>`
返回切片的最后一个和所有其余元素，如果为空，则返回`None`。

```rust
let x = &[0, 1, 2];

if let Some((last, elements)) = x.split_last() {
    assert_eq!(last, &2);
    assert_eq!(elements, &[0, 1]);
}
```

- split_last_mut

`pub fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>`
返回切片的最后一个指针和所有其余元素，如果为空，则返回`None`。

```rust
let x = &mut [0, 1, 2];

if let Some((last, elements)) = x.split_last_mut() {
    *last = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[4, 5, 3]);
```

- last

`pub fn last(&self) -> Option<&T>`
返回切片的最后一个元素，如果为空，则返回`None`。

```rust
let v = [10, 40, 30];
assert_eq!(Some(&30), v.last());

let w: &[i32] = &[];
assert_eq!(None, w.last());
```

- last_mut

`pub fn last_mut(&mut self) -> Option<&mut T>`
返回指向切片中最后一项的可变指针。

```rust
let x = &mut [0, 1, 2];

if let Some(last) = x.last_mut() {
    *last = 10;
}
assert_eq!(Some(&10), x.last());
assert_eq!(x, &[0, 1, 10]);
```

- get

```rust
pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
where
    I: SliceIndex<[T]>,
```

返回对元素或子片段的引用，具体取决于索引的类型。

- 如果给定一个位置，则返回对该位置元素的引用，如果超出范围则返回`None`。
- 如果给定范围，则返回与该范围对应的子切片，如果超出范围，则返回`None`。

```rust
let v = [10, 40, 30];
assert_eq!(Some(&40), v.get(1));
assert_eq!(Some(&[10, 40][..]), v.get(0..2));
assert_eq!(None, v.get(3));
assert_eq!(None, v.get(0..4));
```

- get_mut

```rust
pub fn get_mut<I>(
    &mut self, 
    index: I
) -> Option<&mut <I as SliceIndex<[T]>>::Output> 
where
    I: SliceIndex<[T]>, 
```

返回对元素或子片段的可变引用，具体取决于索引的类型（请参阅get），如果索引超出范围，则返回`None`。

```rust
let x = &mut [0, 1, 2];

if let Some(elem) = x.get_mut(1) {
    *elem = 42;
}
assert_eq!(x, &[0, 42, 2]);
```

- get_unchecked

```rust
pub unsafe fn get_unchecked<I>(
    &self, 
    index: I
) -> &<I as SliceIndex<[T]>>::Output 
where
    I: SliceIndex<[T]>, 
```

返回对元素或子片的引用，而不进行边界检查。 一般不推荐这样做，请谨慎使用！有关安全的选择，请参阅获取。

```rust
let x = &[1, 2, 4];

unsafe {
    assert_eq!(x.get_unchecked(1), &2);
}
```

- get_unchecked_mut

```rust

```

- as_ptr

```rust

```

- as_mut_ptr

```rust

```

- swap

```rust

```

- reverse

```rust

```

- iter

```rust

```

- iter_mut

```rust

```

- windows

```rust

```

- chunks

```rust

```

- chunks_mut

```rust

```

- exact_chunks

```rust

```

- exact_chunks_mut

```rust

```

- split_at

```rust

```

- split_at_mut

```rust

```

- split

```rust

```

- split_mut

```rust

```

- rsplit

```rust

```

- rsplit_mut

```rust

```

- splitn

```rust

```

- splitn_mut

```rust

```

- rsplitn

```rust

```

- rsplitn_mut

```rust

```

- contains

```rust

```

- starts_with

```rust

```

- ends_with

```rust

```

- binary_search

```rust

```

- binary_search_by

```rust

```

- binary_search_by_key

```rust

```

- sort_unstable

```rust

```

- sort_unstable_by

```rust

```

- sort_unstable_by_key

```rust

```

- rotate_left

```rust

```

- rotate_right

```rust

```

- clone_from_slice

```rust

```

- copy_from_slice

```rust

```

- swap_with_slice

```rust

```

- align_to

```rust

```

- align_to_mut

```rust

```

- is_ascii

```rust

```

- eq_ignore_ascii_case

```rust

```

- make_ascii_uppercase

```rust

```

- make_ascii_lowercase

```rust

```

- sort

```rust

```

- sort_by

```rust

```

- sort_by_key

```rust

```

- sort_by_cached_key

```rust

```

- to_vec

```rust

```

- into_vec

```rust

```

- repeat

```rust

```

- to_ascii_uppercase

```rust

```

- to_ascii_lowercase

```rust

```


片段拥有`&[T]`类型。当我们涉及到泛型时会讨论这个`T`。
你可以在[标准库文档](http://doc.rust-lang.org/nightly/std/primitive.slice.html)中找到更多关于slices的文档。

> 元组（tuple 加字面量声明）

具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。

> 指针（\*）

最底层的是裸指针`*const T`和`*mut T`，但解引用它们是不安全的，必须放到`unsafe`块里。
函数：具有函数类型的模式实质上是一个函数指针。

> 元类型

即`()`，其唯一的值也是`()`。

# 模式绑定

## 常规绑定

`Rust` 通过 `let` 关键字进行`模式`绑定。

Rust 是一个静态类型语言，这意味着我们需要先确定我们需要的类型。`Rust`有一个叫做`类型推断`的功能。如果它能确认这是什么类型，`Rust` 不需要你明确地指出来，如下文的`a1`。

因为数值类型中类型有细分，所以使用`let` 做`模式`绑定的时候，会有默认的类型`i32`，也可以用`类型注解（Type annotations）`指定类型，即在原有的赋值模式后面加上`:T`，即英文冒号加上具体类型，如下文的`a2`。

```rust
// 代码在src/demo01.rs
fn main() {
    let a1 = 5;//let 绑定 整数模式默认类型推断是 i32
    let a2: i32 = 5;//也可以不使用默认类型，直接指定类型
    println!("a1 is {} ,a2 is {},are they equal? {}", a1, a2, a1 == a2);//a1 is 5 ,a2 is 5,are they equal? true
    assert_eq!(a1, a2);//断言如果不正确程序会报错，不再继续执行，直接退出

    let b1: u32 = 5;//也可以不使用默认类型，直接指定类型
    // 如果一个模式 没有被使用过，会出现警告，警告是不会阻塞程序编译
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
```

这里的 `assert_eq!` 宏的作用是判断两个参数是不是绝对相等的，即会判断两个值的数据类型和字面值是否都相等。

> 在每个例子中都写上`fn main() {`有点冗长，所以之后我们将省略它。如果你是一路看过来的，确保你写了`main()`函数，而不是省略不写。否则，你将得到一个错误。

## 模式（Patterns）

在许多语言中，上文中的`a1`叫做`变量`，不过 `Rust` 的`变量绑定`有一些不同的巧妙之处。

例如`let`语句的左侧是一个`模式`，即`let` 表达式实际上是一种`模式匹配`，而不仅仅是一个`变量`，这意味着我们可以这样写：

```rust
// 代码在src/demo02.rs
fn main() {
    let (x, y) = (1, 2);
    println!("x = {}, y = {},are they equal? {}", x, y,x == y);//x = 1, y = 2,are they equal? false
    // assert_eq!(x, y);
    // thread 'main' panicked at 'assertion failed: `(left == right)`
    // left: `1`,
    // right: `2`', demo02.rs:4:5
    // note: Run with `RUST_BACKTRACE=1` for a backtrace.
}
```

这里声明使用了 `bool`类型，只有`true`和`false`两个值，通常用来做`逻辑判断`的类型。

在这个语句被计算后，`x`将会是`1`，而`y`将会是`2`。模式非常强大，我们现在还不需要这些功能，所以接下来你只需记住有这个东西就行了。

## 可变绑定

`rust` 在声明`模式`时，绑定默认是`不可变的（immutable）`,即赋予的值的变量，是不可以再次赋值的。

如果需要变量是可以再次赋值的，在`模式`前面加入 `mut`关键字，模式就会成为`可变绑定`的`模式`。

```rust
// 代码在src/demo03.rs

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

```

最开始声明了一个不可以再赋值的`a`和一个可以再赋值的`b`，这个时候对两个值做值匹配断言，结果是`false`。

后面将可以再赋值的`b`重新赋值为`true`，对两个值做值匹配断言，结果是`true`。

后面将可以再赋值的`b`重新置为不可以再次赋值的变量，注意这个时候编译会提出警告，你重置了变量，但是不影响后续编译，这个再对`b`进行赋值，编辑器会直接报错，说不可变的变量不能再次赋值。

## 原始数据赋值实例

```rust
// 代码在src/demo04.rs
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
```

> 有几点是需要特别注意的

- 数值类型可以使用\_分隔符来增加可读性。

- `Rust`还支持单字节字符`b'H'`以及单字节字符串`b"Hello"`，仅限制于`ASCII`字符。 此外，还可以使用`r#"..."#`标记来表示原始字符串，不需要对特殊字符进行转义。

- 使用`&`符号将`String`类型转换成`&str`类型很廉价， 但是使用`to_string()`方法将`&str`转换到`String`类型涉及到分配内存， 除非很有必要否则不要这么做。

- 数组的长度是不可变的，动态的数组称为`Vec (vector)`，可以使用宏`vec!`创建。

- 元组可以使用`==`和`!=`运算符来判断是否相同。不多于`32`个元素的数组和不多于`12`个元素的元组在值传递时是自动复制的。

- `Rust`不提供原生类型之间的隐式转换，只能使用`as`关键字显式转换。

- 可以使用`type`关键字定义某个类型的别名，并且应该采用`驼峰`命名法。

```rust
// explicit conversion
let decimal = 65.4321_f32;
let integer = decimal as u8;
let character = integer as char;

// type aliases
type NanoSecond = u64;
type Point = (u8, u8);`
```

## 初始化绑定（Initializing bindings）

`Rust` 变量绑定有另一个不同于其它语言的方面,绑定要求在可以使用它之前必须`初始化`。

在未使用未赋值变量的时候,`Rust` 只是会警告我们从未使用过这个变量绑定，但是因为我们从未用过它，无害不罚。

然而，如果你确实想使用x，事情就不一样了,`Rust` 会直接报错。

```rust
// 代码在src/demo05.rs
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
```

## 作用域（Scope）

`Rust`变量绑定有一个作用域，一个块是一个被`{`和`}`包围的语句集合，他们被限制只能在他们被定义的块中存在，函数定义也是块。

在下面的例子中我们定义了两个变量绑定，`x`和`y`，他们位于不同的作用域中。`x`可以在`fn main() {}`块中被访问，而`y`只能在`内部块`内访问。

```rust
// 代码在src/demo06.rs
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
```

第一个`println!`将会打印`The value of x is 17 and the value of y is 3`，不过这个并不能编译成功，因为第二个`println!`并不能访问`y`的值，因为它已不在作用域中，相反我们得到错误提示。

## 隐藏（shadowing）

`Rust`变量可以被隐藏。这意味着一个后声明的并位于同一作用域的相同名字的变量绑定将会覆盖前一个变量绑定。

`隐藏`和`可变绑定`可能表现为同一枚硬币的两面，他们是两个不同的概念，不能互换使用。

举个例子，`隐藏`允许我们将一个名字`重绑`定为不同的`类型`，它也可以改变一个绑定的可变性。

注意`隐藏`并不`改变`和`销毁`被绑定的值，这个值会在离开作用域之前继续存在，即便无法通过任何手段`访问`到它。

```rust
// 代码在src/demo07.rs
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
```