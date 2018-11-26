# [primitive-type-bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

## 简介

`bool`表示一个值，该值只能为`true`或`false`。如果将`bool`转换为整数，则`true`将为`1`，`false`将为`0`。

## 完整说明

`bool`实现了各种特性，比如[BitAnd](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html)，[BitOr](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html)，[Not](https://doc.rust-lang.org/nightly/std/ops/trait.Not.html)等，它们允许我们使用`&`，`|`和`!`来执行布尔运算。如果总是要求`bool`值。可以用[assert!](https://doc.rust-lang.org/nightly/std/macro.assert.html)作为测试中的一个重要宏，检查表达式是否返回`true`。

## 基础使用

可以到[rust 实验室](https://play.rust-lang.org/)，运行代码测试。

```rust
//基础使用案例
let bool_val = true & false | false;
assert!(!bool_val);

//综合使用案例
let praise_the_borrow_checker = true;

// 使用if条件判断
if praise_the_borrow_checker {
    println!("oh, yeah!");
} else {
    println!("what?!!");
}

// 或者使用match模式匹配
match praise_the_borrow_checker {
    true => println!("keep praising!"),
    false => println!("you should praise!"),
}

//https://play.rust-lang.org/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Aassert_eq!(true%20as%20i32%2C%201)%3B%0Aassert_eq!(false%20as%20i32%2C%200)%3B%0A%7D
//类型转换
assert_eq!(true as i32, 1);//true
assert_eq!(false as i32, 0);//true
```

## 默认方法

### 继承 [Debug](https://doc.rust-lang.org/nightly/std/fmt/trait.Debug.html)的方法

- [fmt](https://doc.rust-lang.org/nightly/std/fmt/struct.Formatter.html)

`fn fmt(&self, f: &mut Formatter) -> Result<(), Error>`
使用给定的格式化程序格式化值。(TODO：之后再研究具体使用)

```rust
//基础案例
  let bool_val = true;

  assert_eq!(
    "right is: true, error is: false",
    format!("right is: {}, error is: {}", bool_val,!bool_val)
  );
  println!("{:?}", format!("right is: {}, error is: {}", bool_val,!bool_val));

//Deriving an implementation
  #[derive(Debug)]
  struct CheckBool {
    right: bool,
    error: bool,
  }

  let origin = CheckBool {
    right: true,
    error: false,
  };

  println!("The origin is: {:#?}", origin);

//Manually implementing
  use std::fmt;

  struct CheckBool {
    right: bool,
    error: bool,
  }
  impl fmt::Debug for CheckBool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "right is: {}, error is: {}", self.right, self.error)
    }
  }

  let origin = CheckBool {
    right: true,
    error: false,
  };

  println!("{:?}", origin);
  assert_eq!(
    "right is: true, error is: false".to_owned(),
    format!("{:?}", origin)
  );
```

### impl [Clone](https://doc.rust-lang.org/nightly/std/fmt/trait.Clone.html) for bool

- [clone](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html#tymethod.clone)

`fn clone(&self) -> bool`
返回值的副本。

```rust
  let bool_val = true;
  assert_eq!(bool_val.clone(), true);//true
  println!("bool_val is {}", bool_val.clone());//bool_val is true
```

- [clone_form](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html#method.clone_from)

`fn clone_from(&mut self, source: &Self)`
`a.clone_from(&b)`在功能上等同于`a = b.clone()`，但可以重写以重用`a`的资源以避免不必要的分配。请注意这个方法是直接执行，没有返回值的。

```rust
  let bool_right = true;
  let mut bool_val = false;
  bool_val.clone_from(&bool_right);
  assert_eq!(bool_val, true);//true
  println!("bool_val is {}", bool_val);//bool_val is true
```

### impl [PartialOrd](https://doc.rust-lang.org/nightly/std/fmt/trait.PartialOrd.html)<bool> for bool

- [partial_cmp](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

`fn partial_cmp(&self, other: &bool) -> Option<Ordering>`
此方法返回 `self` 和其他值之间的排序（如果存在），对于两个`bool`，主要是存在判断两个是否相等，当然也可以判断大小，`true`对应的是数字 1，`false`对应数字 0，所以`bool_right.partial_cmp(&bool_error)`得到的结论是`Greater`。

```rust
  use std::cmp::Ordering;
  let bool_right = true;
  let bool_error = false;
  let bool_val = true;

  let result = bool_right.partial_cmp(&bool_val);
  let result02 = bool_right.partial_cmp(&bool_error);
  // assert_eq!(result, Some(Ordering::Less));
  // assert_eq!(result, Some(Ordering::Greater));
  // assert_eq!(result, None);
  assert_eq!(result, Some(Ordering::Equal));
  println!("the result is {:#?}",result);//the result is Some(Equal)
  assert_eq!(result02, Some(Ordering::Greater));
  print!("true is {:#?} than false",result02);//true is Some(Greater) than false
```

- [lt](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html#method.lt)

`fn lt(&self, other: &Rhs) -> bool`

通过`<`判断两个数之间关系。

```rust
  let bool_val = 1 < 2;
  assert_eq!(bool_val, true);
  println!("bool_val is {}", bool_val);//bool_val is true
```

- [le](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html#method.le)

`fn le(&self, other: &Rhs) -> bool`

通过`<=`判断两个数之间关系。

```rust
  let bool_val = 2 <= 2;
  assert_eq!(bool_val, true);
  println!("bool_val is {}", bool_val);//bool_val is true
```

- [gt](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html#method.gt)

`fn gt(&self, other: &Rhs) -> bool`

通过`>`判断两个数之间关系。

```rust
  let bool_val = 2 > 2;
  assert_eq!(bool_val, false);
  println!("bool_val is {}", bool_val);//bool_val is false
```

- [ge](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html#method.ge)

`fn ge(&self, other: &Rhs) -> bool`

通过`>=`判断两个数之间关系。

```rust
  let bool_val = 2 >= 2;
  assert_eq!(bool_val, true);
  println!("bool_val is {}", bool_val);//bool_val is true
```

### impl [Ord](https://doc.rust-lang.org/nightly/std/fmt/trait.Debug.html) for bool

- [cmp](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#tymethod.cmp)

`fn cmp(&self, other: &bool) -> Ordering`
此方法返回`self`和`other`之间的`Ordering`。

```rust
  use std::cmp::Ordering;
  let bool_right = true;
  let bool_error = false;
  let bool_val = true;

  let result = bool_right.cmp(&bool_val);
  let result02 = bool_right.cmp(&bool_error);
  // assert_eq!(result, Some(Ordering::Less));
  // assert_eq!(result, Some(Ordering::Greater));
  // assert_eq!(result, None);
  assert_eq!(result, Ordering::Equal);
  println!("the result is {:#?}",result);//the result is Equal
  assert_eq!(result02, Ordering::Greater);
  print!("true is {:#?} than false",result02);//true is Greater than false
```

- [max](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#method.max)

`fn max(self, other: Self) -> Self`
比较并返回两个值的最大值。

```rust
  let bool_right = true;
  let bool_error = false;
  let result = bool_right.max(bool_error);
  assert_eq!(result, true);
  println!("the Greater in true and false is {:#?}",result);//the Greater in true and false is true
```

- [min](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#method.min)

`fn min(self, other: Self) -> Self`
比较并返回两个值的最小值。

```rust
  let bool_right = true;
  let bool_error = false;
  let result = bool_right.min(bool_error);
  assert_eq!(result, false);
  print!("the Less in true and false is {:#?}",result);//the Less in true and false is false
```

### impl<'a> [BitXor](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html)<&'a bool> for bool

- [bitxor](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html#tymethod.bitxor)

```rust
type Output = <bool as BitXor<bool>>::Output

fn bitxor(self, other: bool) -> <bool as BitXor<bool>>::Output
```

应用`^`运算符后的结果类型,(TODO：之后再研究具体使用)

```rust
  use std::ops::BitXor;

  #[derive(Debug, PartialEq)]
  struct Scalar(bool);

  impl BitXor for Scalar {
      type Output = Self;

      // rhs is the "right-hand side" of the expression `a ^ b`
      fn bitxor(self, rhs: Self) -> Self {
          Scalar(self.0 ^ rhs.0)
      }
  }

  assert_eq!(Scalar(true) ^ Scalar(true), Scalar(false));
  assert_eq!(Scalar(true) ^ Scalar(false), Scalar(true));
  assert_eq!(Scalar(false) ^ Scalar(true), Scalar(true));
  assert_eq!(Scalar(false) ^ Scalar(false), Scalar(false));

  let bool_right = true;
  let bool_error = false;
  // assert_eq!(result, false);
  println!("true after applying the ^ operator with true is {:#?}",bool_right^bool_right);//true after applying the ^ operator with true is false
  println!("true after applying the ^ operator with false is {:#?}",bool_right^bool_error);//true after applying the ^ operator with false is true
  println!("false after applying the ^ operator with true is {:#?}",bool_error^bool_right);//false after applying the ^ operator with true is true
  println!("false after applying the ^ operator with false is {:#?}",bool_error^bool_error);//false after applying the ^ operator with false is false
```

### impl [BitXor](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html)<bool> for bool

- [bitxor](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html#tymethod.bitxor)

```rust
type Output = bool

fn bitxor(self, other: bool) -> bool
```

应用`^`运算符后的结果类型。

```rust
  let bool_right = true;
  let bool_error = false;
  // assert_eq!(result, false);
  println!("true after applying the ^ operator with true is {:#?}",bool_right^bool_right);//true after applying the ^ operator with true is false
  println!("true after applying the ^ operator with false is {:#?}",bool_right^bool_error);//true after applying the ^ operator with false is true
  println!("false after applying the ^ operator with true is {:#?}",bool_error^bool_right);//false after applying the ^ operator with true is true
  println!("false after applying the ^ operator with false is {:#?}",bool_error^bool_error);//false after applying the ^ operator with false is false
```

### impl<'a> [BitXor](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html)<bool> for &'a bool

- [bitxor](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html#tymethod.bitxor)

```rust
type Output = <bool as BitXor<bool>>::Output

fn bitxor(self, other: &'a bool) -> <bool as BitXor<bool>>::Output
```

应用`^`运算符后的结果类型。(TODO：之后再研究具体使用)

```rust
  let bool_right = true;
  let bool_error = false;
  // assert_eq!(result, false);
  println!("true after applying the ^ operator with true is {:#?}",bool_right^bool_right);//true after applying the ^ operator with true is false
  println!("true after applying the ^ operator with false is {:#?}",bool_right^bool_error);//true after applying the ^ operator with false is true
  println!("false after applying the ^ operator with true is {:#?}",bool_error^bool_right);//false after applying the ^ operator with true is true
  println!("false after applying the ^ operator with false is {:#?}",bool_error^bool_error);//false after applying the ^ operator with false is false
```

### impl<'a, 'b> [BitXor](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html)<&'a bool> for &'b bool

- [bitxor](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html#tymethod.bitxor)

```rust
type Output = <bool as BitXor<bool>>::Output

fn bitxor(self, other: &'a bool) -> <bool as BitXor<bool>>::Output
```

应用`^`运算符后的结果类型。(TODO：之后再研究具体使用)

```rust
  let bool_right = true;
  let bool_error = false;
  // assert_eq!(result, false);
  println!("true after applying the ^ operator with true is {:#?}",bool_right^bool_right);//true after applying the ^ operator with true is false
  println!("true after applying the ^ operator with false is {:#?}",bool_right^bool_error);//true after applying the ^ operator with false is true
  println!("false after applying the ^ operator with true is {:#?}",bool_error^bool_right);//false after applying the ^ operator with true is true
  println!("false after applying the ^ operator with false is {:#?}",bool_error^bool_error);//false after applying the ^ operator with false is false
```

### impl [Eq](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html) for boolimpl PartialEq<bool> for bool

通过`==`判断两个值是否相等，感觉和`cmp`和`partial_cmp`功能上有一定相同。

```rust
  let bool_right = true;
  let bool_error = false;
  assert_eq!(bool_right == bool_error, false);
  println!("true is not same as false"); //true is not same as false
```

### impl [PartialEq](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html)<bool> for bool

此特征可与`#[derive]`一起使用。在结构上派生(`derive`)时，如果所有字段相等，则两个实例相等，如果任何字段不相等则不相等。当在枚举上导出(`derive`)时，每个变体等于其自身并且不等于其他变体。

- [eq](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html#tymethod.eq)

`fn eq(&self, other: &bool) -> bool`

此方法测试`self`和其他值是否相等，并由`==`使用。

```rust
  let bool_right = true;
  let bool_error = false;
  assert_eq!(bool_right.eq(&bool_error), false);
  println!("true is not same as false"); //true is not same as false
```

- [ne](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html#method.ne)

`fn ne(&self, other: &bool) -> bool`
此方法测试`self`和其他值是否不相等，并由`!=`使用。

```rust
  let bool_right = true;
  let bool_error = false;
  assert_eq!(bool_right.ne(&bool_error), true);
  println!("true is not same as false"); //true is not same as false
```

### impl<'a> Not for &'a bool

- [not](https://doc.rust-lang.org/nightly/std/ops/trait.Not.html#tymethod.not)

```rust
type Output = <bool as Not>::Output
//The resulting type after applying the ! operator.

fn not(self) -> <bool as Not>::Output
```

执行一元`!`即取反操作。(TODO：之后再研究具体使用)

```rust
  let bool_right = true;
  let bool_error = false;
  assert_eq!(!bool_right, false);//true
  println!("unary false is {}",!bool_error); //unary false is true
```

### impl Not for bool

- [not](https://doc.rust-lang.org/nightly/std/ops/trait.Not.html#tymethod.not)

```rust
type Output = bool

fn not(self) -> bool
```

执行一元`!`即取反操作。

```rust
  let bool_right = true;
  let bool_error = false;
  assert_eq!(!bool_right, false);//true
  println!("unary false is {}",!bool_error); //unary false is true
```

### impl [Copy](https://doc.rust-lang.org/nightly/std/marker/trait.Copy.html) for bool

> 与`Clone`的区别

```text
Copies happen implicitly, for example as part of an assignment y = x. The behavior of Copy is not overloadable; it is always a simple bit-wise copy.
```

拷贝隐式发生，例如作为赋值`y = x`的一部分。 `Copy`的行为不可重载;它始终是一个简单的逐位复制。

```text
Cloning is an explicit action, x.clone(). The implementation of Clone can provide any type-specific behavior necessary to duplicate values safely. For example, the implementation of Clone for String needs to copy the pointed-to string buffer in the heap. A simple bitwise copy of String values would merely copy the pointer, leading to a double free down the line. For this reason, String is Clone but not Copy.
```

克隆是一个显式操作，`x.clone()`。 `Clone`的实现可以提供`安全复制`值所需的任何特定于类型的行为。例如，`Clone` for `String`的实现需要复制堆中指向的字符串缓冲区。 `String`值的简单按位复制只会复制指针，从而导致行中的`双重释放`。因此，`String`是`Clone`但不是`Copy`。

```text
Clone is a supertrait of Copy, so everything which is Copy must also implement Clone. If a type is Copy then its Clone implementation only needs to return *self (see the example above).
```

`Clone`是一个`Copy`的超级，所以`Copy`的一切也必须实现`Clone`。如果类型是`Copy`，那么它的`Clone`实现只需要返回`*self`。

```text
Generally speaking, if your type can implement Copy, it should. Keep in mind, though, that implementing Copy is part of the public API of your type. If the type might become non-Copy in the future, it could be prudent to omit the Copy implementation now, to avoid a breaking API change.
```

一般来说，如果你的类型可以实现`Copy`，它应该。但请记住，实现`Copy`是您的类型的公共`API`的一部分。如果该类型将来可能变为`non-Copy`，则可以谨慎地省略现在的`Copy`实现，以避免 API 更改中断。

```rust
#[derive(Debug)]
struct Foo;

let x = Foo;

let y = x;

// `x` has moved into `y`, and so cannot be used

println!("{:?}", x); // error: use of moved value
                      //^ value used here after move

// We can derive a `Copy` implementation. `Clone` is also required, as it's
// a supertrait of `Copy`.
#[derive(Debug, Copy, Clone)]
struct Foo;

let x = Foo;

let y = x;

// `y` is a copy of `x`

println!("{:?},{:?}", x, y); // Foo,Foo
```

### impl [BitAnd](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html)<bool> for bool

for bool

- [bitand](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html#tymethod.bitand)

```rust
type Output = bool

fn bitand(self, rhs: bool) -> bool
```

应用`&`运算符后的结果类型。

```rust
  let bool_right = true;
  let bool_error = false;
  println!("true after applying the & operator with true is {:#?}",bool_right&bool_right);//true after applying the & operator with true is true
  println!("true after applying the & operator with false is {:#?}",bool_right&bool_error);//true after applying the & operator with false is false
  println!("false after applying the & operator with false is {:#?}",bool_error&bool_error);//false after applying the & operator with false is false
```

### impl<'a, 'b> [BitAnd](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html)<&'a bool> for &'b bool

- [bitand](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html#tymethod.bitand)

```rust
type Output = <bool as BitAnd<bool>>::Output
//The resulting type after applying the & operator.
fn bitand(self, other: &'a bool) -> <bool as BitAnd<bool>>::Output
```

应用`&`运算符后的结果类型。(TODO：之后再研究具体使用)

```rust
  let bool_right = true;
  let bool_error = false;
  println!("true after applying the & operator with true is {:#?}",bool_right&bool_right);//true after applying the & operator with true is true
  println!("true after applying the & operator with false is {:#?}",bool_right&bool_error);//true after applying the & operator with false is false
  println!("false after applying the & operator with false is {:#?}",bool_error&bool_error);//false after applying the & operator with false is false
```

### impl<'a> [BitAnd](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html)<&'a bool> for bool

- [bitand](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html#tymethod.bitand)

```rust
type Output = <bool as BitAnd<bool>>::Output
//The resulting type after applying the & operator.
fn bitand(self, other: &'a bool) -> <bool as BitAnd<bool>>::Output
```

应用`&`运算符后的结果类型。(TODO：之后再研究具体使用)

```rust
  let bool_right = true;
  let bool_error = false;
  println!("true after applying the & operator with true is {:#?}",bool_right&bool_right);//true after applying the & operator with true is true
  println!("true after applying the & operator with false is {:#?}",bool_right&bool_error);//true after applying the & operator with false is false
  println!("false after applying the & operator with false is {:#?}",bool_error&bool_error);//false after applying the & operator with false is false
```

### impl<'a> [BitAnd](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html)<bool> for &'a bool

- [bitand](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html#tymethod.bitand)

```rust
type Output = <bool as BitAnd<bool>>::Output
//The resulting type after applying the & operator.
fn bitand(self, other: &'a bool) -> <bool as BitAnd<bool>>::Output
```

应用`&`运算符后的结果类型。(TODO：之后再研究具体使用)

```rust
  let bool_right = true;
  let bool_error = false;
  println!("true after applying the & operator with true is {:#?}",bool_right&bool_right);//true after applying the & operator with true is true
  println!("true after applying the & operator with false is {:#?}",bool_right&bool_error);//true after applying the & operator with false is false
  println!("false after applying the & operator with false is {:#?}",bool_error&bool_error);//false after applying the & operator with false is false
```

### impl [Display](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) for bool

- [fmt](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html#tymethod.fmt)

`fn fmt(&self, f: &mut Formatter) -> Result<(), Error>`
使用给定的格式化程序格式化值。(TODO：之后再研究具体使用)

```rust
//基础案例
  let bool_val = true;

  assert_eq!(
    "right is: true, error is: false",
    format!("right is: {}, error is: {}", bool_val,!bool_val)
  );
  println!("{:?}", format!("right is: {}, error is: {}", bool_val,!bool_val));

//Deriving an implementation
  #[derive(Debug)]
  struct CheckBool {
    right: bool,
    error: bool,
  }

  let origin = CheckBool {
    right: true,
    error: false,
  };

  println!("The origin is: {:#?}", origin);

//Manually implementing
  use std::fmt;

  struct CheckBool {
    right: bool,
    error: bool,
  }
  impl fmt::Debug for CheckBool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "right is: {}, error is: {}", self.right, self.error)
    }
  }

  let origin = CheckBool {
    right: true,
    error: false,
  };

  println!("{:?}", origin);
  assert_eq!(
    "right is: true, error is: false".to_owned(),
    format!("{:?}", origin)
  );
```

### impl [Default](https://doc.rust-lang.org/nightly/std/default/trait.Default.html) for bool

- [default](https://doc.rust-lang.org/nightly/std/default/trait.Default.html#tymethod.default)

`fn default() -> bool`

返回默认值`false`

```rust
  let bool_val: bool = Default::default();
  println!("the default value of bool is {:#?}",bool_val);//the default value of bool is false
```

### impl [FromStr](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) for bool

- [from_str](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html#tymethod.from_str)

```rust
type Err = ParseBoolError
// The associated error which can be returned from parsing.
fn from_str(s: &str) -> Result<bool, ParseBoolError>
```

从字符串中解析`bool`。 产生一个结果`<bool，ParseBoolError>`，因为 s 实际上可能是也可能不是可解析的。

```rust
use std::str::FromStr;

assert_eq!(FromStr::from_str("true"), Ok(true));
assert_eq!(FromStr::from_str("false"), Ok(false));
assert!(<bool as FromStr>::from_str("not even a boolean").is_err());

// Note, in many cases, the .parse() method on str is more proper.
//注意，在很多情况下，str上的.parse（）方法更合适。
assert_eq!("true".parse(), Ok(true));
assert_eq!("false".parse(), Ok(false));
assert!("not even a boolean".parse::<bool>().is_err());
```

### impl [Hash](https://doc.rust-lang.org/nightly/std/hash/trait.Hash.html) for bool

- [hash](https://doc.rust-lang.org/nightly/std/hash/trait.Hash.html#tymethod.hash)

```rust
fn hash<H>(&self, state: &mut H)
where
    H: Hasher
```

将此值提供给给定的`[Hasher]`。(TODO：之后再研究具体使用)

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

let mut hasher = DefaultHasher::new();
7920.hash(&mut hasher);
println!("Hash is {:x}!", hasher.finish());//Hash is fa637a3502273993!
```

- [hash_slice](https://doc.rust-lang.org/nightly/std/hash/trait.Hash.html#method.hash_slice)

```rust
//1.3.0 version
fn hash_slice<H>(data: &[Self], state: &mut H)
where
    H: Hasher
```

在给定的`[Hasher]`中提供这种类型的切片。(TODO：之后再研究具体使用)

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

let mut hasher = DefaultHasher::new();
let numbers = [6, 28, 496, 8128];
Hash::hash_slice(&numbers, &mut hasher);
println!("Hash is {:x}!", hasher.finish());
```

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/std/ops/trait.BitXorAssign.html)<bool> for bool1.8.0

- [bitxor_assign](https://doc.rust-lang.org/nightly/std/ops/trait.BitXorAssign.html#tymethod.bitxor_assign)

`fn bitxor_assign(&mut self, other: bool)`

执行`^=`操作。

```rust
use std::ops::BitXorAssign;

#[derive(Debug, PartialEq)]
struct Personality {
    has_soul: bool,
    likes_knitting: bool,
}

impl BitXorAssign for Personality {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.has_soul ^= rhs.has_soul;
        self.likes_knitting ^= rhs.likes_knitting;
    }
}

let mut personality = Personality { has_soul: false, likes_knitting: true };
personality ^= Personality { has_soul: true, likes_knitting: true };
assert_eq!(personality, Personality { has_soul: true, likes_knitting: false});

let bool_right = true;
let mut bool_val = false;
bool_val ^= bool_right;
assert_eq!(bool_val, true);
println!(
    "true after applying the ^= operator is {:#?}",
    bool_val
); //true after applying the ^= operator is true
```

### impl<'a> [BitXorAssign](https://doc.rust-lang.org/nightly/std/ops/trait.BitXorAssign.html)<&'a bool> for bool1.22.0

- [bitxor_assign](https://doc.rust-lang.org/nightly/std/ops/trait.BitXorAssign.html#tymethod.bitxor_assign)

`fn bitxor_assign(&mut self, other: bool)`
执行`^=`操作。(TODO：之后再研究具体使用)

```rust
use std::ops::BitXorAssign;

#[derive(Debug, PartialEq)]
struct Personality {
    has_soul: bool,
    likes_knitting: bool,
}

impl BitXorAssign for Personality {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.has_soul ^= rhs.has_soul;
        self.likes_knitting ^= rhs.likes_knitting;
    }
}

let mut personality = Personality { has_soul: false, likes_knitting: true };
personality ^= Personality { has_soul: true, likes_knitting: true };
assert_eq!(personality, Personality { has_soul: true, likes_knitting: false});

```

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/std/ops/trait.BitOrAssign.html)<bool> for bool

- [bitor_assign](https://doc.rust-lang.org/nightly/std/ops/trait.BitOrAssign.html#tymethod.bitor_assign)

`fn bitor_assign(&mut self, other: bool)`
执行`|=`操作。

```rust
//version 1.8.0

let bool_right = true;
let mut bool_val = false;
bool_val |= bool_right;
assert_eq!(bool_val, true);
println!(
    "true after applying the |= operator is {:#?}",
    bool_val
); //true after applying the |= operator is true

```

### impl<'a> [BitOrAssign](https://doc.rust-lang.org/nightly/std/ops/trait.BitOrAssign.html)<&'a bool> for bool

- [bitor_assign](https://doc.rust-lang.org/nightly/std/ops/trait.BitOrAssign.html#tymethod.bitor_assign)

`fn bitor_assign(&mut self, other: bool)`
执行`|=`操作。

```rust
//version 1.22.0

let bool_right = true;
let mut bool_val = false;
bool_val |= bool_right;
assert_eq!(bool_val, true);
println!(
    "true after applying the |= operator is {:#?}",
    bool_val
); //true after applying the |= operator is true

```

### impl<'a> [BitAndAssign](https://doc.rust-lang.org/nightly/std/ops/trait.BitAndAssign.html)<&'a bool> for bool1.22.0

- [bitand_assign](https://doc.rust-lang.org/nightly/std/ops/trait.BitAndAssign.html#tymethod.bitand_assign)

`fn bitxor_assign(&mut self, other: bool)`
执行`&=`操作。

```rust
//version 1.22.0

let bool_right = true;
let mut bool_val = false;
bool_val &= bool_right;
assert_eq!(bool_val, false);
println!(
    "true after applying the &= operator is {:#?}",
    bool_val
); //true after applying the &= operator is false

```

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/std/ops/trait.BitAndAssign.html)<bool> for bool1.8.0

- [bitand_assign](https://doc.rust-lang.org/nightly/std/ops/trait.BitAndAssign.html#tymethod.bitand_assign)

`fn bitxor_assign(&mut self, other: bool)`
执行`&=`操作。(TODO：之后再研究具体使用)

```rust
//version 1.8.0

let bool_right = true;
let mut bool_val = false;
bool_val &= bool_right;
assert_eq!(bool_val, false);
println!(
    "true after applying the &= operator is {:#?}",
    bool_val
); //true after applying the &= operator is false
```

### impl [BitOr](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html)<bool> for bool

- [bitor](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html#tymethod.bitor)

```rust
type Output = bool
The resulting type after applying the | operator.
fn bitor(self, rhs: bool) -> bool
```

执行`|`操作。

```rust
  let bool_right = true;
  let bool_error = false;
  println!("true after applying the | operator with true is {:#?}",bool_right|bool_right);//true after applying the | operator with true is true
  println!("true after applying the | operator with false is {:#?}",bool_right|bool_error);//true after applying the | operator with false is false
  println!("false after applying the | operator with false is {:#?}",bool_error|bool_error);//false after applying the | operator with false is false
```

### impl<'a> [BitOr](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html)<&'a bool> for bool

- [bitor](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html#tymethod.bitor)

```rust
type Output = <bool as BitOr<bool>>::Output
// The resulting type after applying the | operator.
//应用|后得到的类型值
fn bitor(self, other: &'a bool) -> <bool as BitOr<bool>>::Output
```

执行`|`操作。(TODO：之后再研究具体使用)

```rust
  let bool_right = true;
  let bool_error = false;
  println!("true after applying the | operator with true is {:#?}",bool_right|bool_right);//true after applying the | operator with true is true
  println!("true after applying the | operator with false is {:#?}",bool_right|bool_error);//true after applying the | operator with false is false
  println!("false after applying the | operator with false is {:#?}",bool_error|bool_error);//false after applying the | operator with false is false
```

### impl<'a> [BitOr](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html)<bool> for &'a bool

- [bitor](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html#tymethod.bitor)

```rust
type Output = <bool as BitOr<bool>>::Output
// The resulting type after applying the | operator.
//应用|后得到的类型值
fn bitor(self, other: bool) -> <bool as BitOr<bool>>::Output
```

执行`|`操作。(TODO：之后再研究具体使用)

```rust
  let bool_right = true;
  let bool_error = false;
  println!("true after applying the | operator with true is {:#?}",bool_right|bool_right);//true after applying the | operator with true is true
  println!("true after applying the | operator with false is {:#?}",bool_right|bool_error);//true after applying the | operator with false is false
  println!("false after applying the | operator with false is {:#?}",bool_error|bool_error);//false after applying the | operator with false is false
```

### impl<'a, 'b> [BitOr](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html)<&'a bool> for &'b bool

- [bitor](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html#tymethod.bitor)

```rust
type Output = <bool as BitOr<bool>>::Output
// The resulting type after applying the | operator.
//应用|后得到的类型值
fn bitor(self, other: &'a bool) -> <bool as BitOr<bool>>::Output
```

执行`|`操作。(TODO：之后再研究具体使用)

```rust
  let bool_right = true;
  let bool_error = false;
  println!("true after applying the | operator with true is {:#?}",bool_right|bool_right);//true after applying the | operator with true is true
  println!("true after applying the | operator with false is {:#?}",bool_right|bool_error);//true after applying the | operator with false is false
  println!("false after applying the | operator with false is {:#?}",bool_error|bool_error);//false after applying the | operator with false is false
```