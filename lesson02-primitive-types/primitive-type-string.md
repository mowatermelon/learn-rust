# [primitive-type-string](https://doc.rust-lang.org/nightly/std/primitive.str.html)

## 简介

`str`类型（也称为`字符串切片`）是最原始的字符串类型。它通常以借来的形式出现，`&str`和[std::str module](https://doc.rust-lang.org/nightly/std/str/index.html)比较相近，它也是字符串文字的类型，`&'static str`，默认编码类型是`UTF-8`，。

最底层的是不定长类型`str`，更常用的是字符串切片`&str`和堆分配字符串`String`。

其中字符串切片是`静态分配`的，有固定的大小，并且不可变，而堆分配字符串是可变的。

## 完整说明

这里我们声明了一个字符串文字，也称为字符串切片。字符串文字具有静态生命周期，这意味着字符串hello_world保证在整个程序的持续时间内有效。我们也可以明确指定`hello_world`的生命周期：

可以到[rust 实验室](https://play.rust-lang.org/)，运行代码测试。

```rust
//基础使用案例
let hello = "Hello, world!";

// with an explicit type annotation
let hello: &'static str = "Hello, world!";

// A &str is made up of two components: a pointer to some bytes, and a length. You can look at these with the as_ptr and len methods:
use std::slice;
use std::str;

let story = "Once upon a time...";

let ptr = story.as_ptr();
let len = story.len();

// story has nineteen bytes
assert_eq!(19, len);

// We can re-build a str out of ptr and len. This is all unsafe because
// we are responsible for making sure the two components are valid:
let s = unsafe {
    // First, we build a &[u8]...
    let slice = slice::from_raw_parts(ptr, len);

    // ... and then convert that slice into a string slice
    str::from_utf8(slice)
};

assert_eq!(s, Ok(story));
// Note: This example shows the internals of &str. unsafe should not be used to get a string slice under normal circumstances. Use as_slice instead.
```

它们是`'static`的，因为它们直接存储在最终二进制文件中，因此对于`'static`持续时间有效。

## 默认方法

### impl [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)的方法

#### len

`pub const fn len(&self) -> usize`
返回以字节为单位的字符串长度，根据不同[char](https://doc.rust-lang.org/nightly/std/char/index.html)长度对应的字节长度有所不同，这样返回的实际长度和字符串长度有所不同。

```rust
let normal_val = "foo";
let fancy_val = "ƒoo";
assert_eq!(3, normal_val.len());
println!("The length of normal_val is {}", normal_val.len());//The length of normal_val is 3

assert_eq!(4, fancy_val.len());
println!("The length of fancy_val is {}", fancy_val.len());//The length of fancy_val is 4
```

#### is_empty

`pub const fn is_empty(&self) -> bool`
如果字符串的长度为零字节，则返回true，请注意这个不会自动去除字符串中的空格。

```rust
let empty_val = "";
let space_val = "   ";
let full_val = "ƒoo";
assert_eq!(true,empty_val.is_empty());
println!("The length of empty_val is {}", empty_val.len());//The length of empty_val is 0

assert_eq!(false,space_val.is_empty());
println!("The length of space_val is {}", space_val.len());//The length of space_val is 3

assert_eq!(false,full_val.is_empty());
println!("The length of full_val is {}", full_val.len());//The length of full_val is 4
```

#### is_char_boundary

`pub fn is_char_boundary(&self, index: usize) -> bool`

```text
Checks that index-th byte lies at the start and/or end of a UTF-8 code point sequence.

The start and end of the string (when index == self.len()) are considered to be boundaries.

Returns false if index is greater than self.len().
```

检查索引字节位于`UTF-8`代码点序列的开头和/或结尾。 字符串的开头和结尾(当`index == self.len()`)被认为是边界。 如果`index`大于`self.len()`，则返回`false`。不太清楚这个使用场景。

```rust
let check_val = "ƒoo boo";
assert_eq!(true, check_val.is_char_boundary(0));
assert_eq!(true, check_val.is_char_boundary(check_val.len()));
assert_eq!(false, check_val.is_char_boundary(check_val.len() + 1));
assert_eq!(true, check_val.is_char_boundary(5));
println!("The third of check_val is {:?}", check_val.get(3..4)); //The third of check_val is Some("o")
println!("The length of check_val is {}", check_val.len()); //The length of check_val is 8
```

#### as_bytes

`pub const fn as_bytes(&self) -> &[u8]`

将字符串切片转换为字节切片。要将字节切片转换回字符串切片，请使用[str::from_utf8](https://doc.rust-lang.org/nightly/std/str/fn.from_utf8.html)函数。

每个`char`转换成对应的字节编码，`ASCII`码辅助记忆，小a97，大A。

```rust
let check_val = "foo boo";
let bytes = check_val.as_bytes();
assert_eq!(b"foo boo", bytes);
println!("{:#?}",bytes);//[102, 111, 111, 32, 98, 111, 111]
```

#### as_bytes_mut

`pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8]`

将`可变字符串切片`转换为`可变字节切片`。要将`可变字节切片`转换回`可变字符串切片`，请使用[str::from_utf8_mut](https://doc.rust-lang.org/nightly/std/str/fn.from_utf8_mut.html)函数。

每个`char`转换成对应的字节编码，`ASCII`码辅助记忆，`小a97，大A65`，注意调用`as_bytes_mut`需要是一个非字面量声明的模式。

```rust
//version 1.20.0

// let mut err_val = "aAo boo";
// let _byte = unsafe { err_val.as_bytes_mut()};
//                       ^^^^^^^ cannot borrow as mutable 不能借用可变的

//简单版
let mut check_val = String::from("foo boo");
let bytes = unsafe { check_val.as_bytes_mut()};
assert_eq!(b"foo boo", bytes);
println!("{:?}",bytes);//[102, 111, 111, 32, 98, 111, 111]

//复杂版
let mut s = String::from("🗻∈🌏");

unsafe {
    let bytes = s.as_bytes_mut();

    bytes[0] = 0xF0;
    bytes[1] = 0x9F;
    bytes[2] = 0x8D;
    bytes[3] = 0x94;
}

assert_eq!("🍔∈🌏", s);
```

#### as_ptr

`pub const fn as_ptr(&self) -> *const u8`

将`字符串切片`转换为`原始指针`。 由于`字符串切片`是一个`字节切片`，`原始指针`指向[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)。该指针将指向字符串切片的第一个字节。

```rust
let check_val = "aAo boo";
let ptr = check_val.as_ptr();
println!("{:?}",ptr);//0x560c356b2600
```

#### get

```rust
pub fn get<I>(&self, i: I) -> Option<&<I as SliceIndex<str>>::Output> 
where
    I: SliceIndex<str>,
```

返回`str`的子切片。 这是索引`str`的非恐慌替代方案。每当等效的索引操作发生混乱时，返回`None`，`起始边界`为可以忽略，但是`结束边界`必传，否则程序会报错，当索引值超过`str`长度直接返回`0`。

```rust
let check_val = "aAo boo";
assert_eq!(Some("a"), check_val.get(0..1));
assert_eq!(Some("aAo"), check_val.get(0..3));
// indices not on UTF-8 sequence boundaries
//索引不在UTF-8序列边界上
// assert!(check_val.get(1..).is_none());//thread 'main' panicked at 'assertion failed: check_val.get(1..).is_none()'
assert!(check_val.get(..8).is_none());

// 超过边界
assert!(check_val.get(..42).is_none());
```

#### get_mut

```rust
pub fn get_mut<I>(
    &mut self,
    i: I
) -> Option<&mut <I as SliceIndex<str>>::Output>
where
    I: SliceIndex<str>,
```

返回`str`的可变子切片。 这是索引`str`的非恐慌替代方案。每当等效的索引操作发生混乱时，返回`None`，`起始边界`为可以忽略，但是`结束边界`必传，否则程序会报错，当索引值超过`str`长度直接返回`0`。

```rust
let mut v = String::from("hello");
// correct length
assert!(v.get_mut(0..5).is_some());
// out of bounds
assert!(v.get_mut(..42).is_none());
assert_eq!(Some("he"), v.get_mut(0..2).map(|v| &*v));

assert_eq!("hello", v);
{
    let s = v.get_mut(0..2);
    let s = s.map(|s| {
        s.make_ascii_uppercase();
        &*s
    });
    assert_eq!(Some("HE"), s);
}
assert_eq!("HEllo", v);
```

#### get_unchecked

```rust
pub unsafe fn get_unchecked<I>(&self, i: I) -> &<I as SliceIndex<str>>::Output 
where
    I: SliceIndex<str>, 
```

返回`str`的未经检查的子切片。 这是索引`str`的未经检查的替代方法。

> 这个函数的调用者负责满足这些前提条件

1  起始索引必须在结束索引之前;
2  索引必须在原始切片的范围内;
3  索引必须位于UTF-8序列边界上。
4  如果失败，返回的字符串切片可能会引用无效内存或违反str类型传递的不变量。

```rust
let v = "🗻∈🌏";
unsafe {
    assert_eq!("🗻", v.get_unchecked(0..4));
    assert_eq!("∈", v.get_unchecked(4..7));
    assert_eq!("🌏", v.get_unchecked(7..11));
}
```

#### get_unchecked_mut

```rust
pub unsafe fn get_unchecked_mut<I>(
    &mut self, 
    i: I
) -> &mut <I as SliceIndex<str>>::Output 
where
    I: SliceIndex<str>, 
```

返回`str`的可变未经检查的子切片。 这是索引`str`的未经检查的替代方法。

> 这个函数的调用者负责满足这些前提条件

1  起始索引必须在结束索引之前;
2  索引必须在原始切片的范围内;
3  索引必须位于UTF-8序列边界上。
4  如果失败，返回的字符串切片可能会引用无效内存或违反str类型传递的不变量。

```rust
let mut v = String::from("🗻∈🌏");
unsafe {
    assert_eq!("🗻", v.get_unchecked_mut(0..4));
    assert_eq!("∈", v.get_unchecked_mut(4..7));
    assert_eq!("🌏", v.get_unchecked_mut(7..11));
}
```

#### split_at

`pub fn split_at(&self, mid: usize) -> (&str, &str)`

在索引处将一个字符串切片分成两个。 参数`mid`应该是从字符串开头偏移的字节。它还必须位于`UTF-8`代码点的边界上。 返回的两个切片从字符串切片的开头到中间，从中间到字符串切片的末尾。 要获取可变的字符串切片。

> 警告

如果`mid`不在`UTF-8`代码点边界上，或者如果它超出字符串切片的最后一个代码点，则发生混乱。

```rust
let s = "Per Martin-Löf";

let (first, last) = s.split_at(3);

assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);
```

#### split_at_mut

`pub fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str)`

在索引处将一个字符串切片分成两个。 参数`mid`应该是从字符串开头偏移的字节。它还必须位于`UTF-8`代码点的边界上。 返回的两个切片从字符串切片的开头到中间，从中间到字符串切片的末尾。

> 警告

如果`mid`不在`UTF-8`代码点边界上，或者如果它超出字符串切片的最后一个代码点，则发生混乱。

```rust
let mut s = "Per Martin-Löf".to_string();
{
    let (first, last) = s.split_at_mut(3);
    first.make_ascii_uppercase();
    assert_eq!("PER", first);
    assert_eq!(" Martin-Löf", last);
}
assert_eq!("PER Martin-Löf", s);
```

#### chars

`pub fn chars(&self) -> Chars`

返回字符串切片的字符上的迭代器。 由于字符串切片由有效的`UTF-8`组成，我们可以通过`char`迭代字符串切片。此方法返回此类迭代器。 重要的是要记住`char`表示`Unicode`标量值，并且可能与您对`字符`的概念不符。对字形集群的迭代可能就是你真正想要的。

```rust
let word = "goodbye";

let count = word.chars().count();
assert_eq!(7, count);

let mut chars = word.chars();

assert_eq!(Some('g'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('d'), chars.next());
assert_eq!(Some('b'), chars.next());
assert_eq!(Some('y'), chars.next());
assert_eq!(Some('e'), chars.next());

assert_eq!(None, chars.next());

//Remember, chars may not match your human intuition about characters:

let y = "y̆";

let mut chars = y.chars();

assert_eq!(Some('y'), chars.next()); // not 'y̆'
assert_eq!(Some('\u{0306}'), chars.next());

assert_eq!(None, chars.next());
```

#### char_indices

`pub fn char_indices(&self) -> CharIndices`

返回字符串切片的字符及其位置的迭代器。 由于字符串切片由有效的UTF-8组成，我们可以通过char迭代字符串切片。此方法返回这两个字符的迭代器及其字节位置。 迭代器产生元组。位置是第一，`char`是第二。

```rust
let word = "goodbye";

let count = word.char_indices().count();
assert_eq!(7, count);

let mut char_indices = word.char_indices();

assert_eq!(Some((0, 'g')), char_indices.next());
assert_eq!(Some((1, 'o')), char_indices.next());
assert_eq!(Some((2, 'o')), char_indices.next());
assert_eq!(Some((3, 'd')), char_indices.next());
assert_eq!(Some((4, 'b')), char_indices.next());
assert_eq!(Some((5, 'y')), char_indices.next());
println!("{:?}",char_indices.next());//Some((6, 'e'))
assert_eq!(None, char_indices.next());

//Remember, chars may not match your human intuition about characters:

let yes = "y̆es";

let mut char_indices = yes.char_indices();

assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
assert_eq!(Some((1, '\u{0306}')), char_indices.next());

// note the 3 here #### the last character took up two bytes
assert_eq!(Some((3, 'e')), char_indices.next());
assert_eq!(Some((4, 's')), char_indices.next());

assert_eq!(None, char_indices.next());
```
