# learn_rust_algorithms

学习rust algorithms的打怪过程

# 冒泡排序

## 代码路径

https://coding.net/u/melonHero/p/learn-rust/git/blob/algorithms/src/bubble_sort.rs

## 实现代码

```rust
pub struct Bubbles {}

impl Bubbles {
    pub fn rank(list: Vec<i32>) -> Vec<i32> {
        let mut clone_list = list;
        let hi = clone_list.len() as i32 - 1i32;
        for i in 0..(hi - 1) {
            for j in 0..(hi - 1) {
                let temp_index = j as usize;
                if clone_list[temp_index] > clone_list[temp_index + 1] {
                    let mut temp = clone_list[temp_index];
                    clone_list[temp_index] = clone_list[temp_index + 1];
                    clone_list[temp_index + 1] = temp;
                }
            }
        }
        return clone_list;
    }
}
```

## 使用方法

```rust
mod bubble_sort;

use bubble_sort::Bubbles;

fn main() {
    println!("{}", "hello");
    let will_sort: Vec<i32> = vec![7,6,5,4,3,2,1];
    let mut _tmp: Vec<i32> = Bubbles::rank(will_sort);

    for i in _tmp.into_iter() {
        println!("{}", i);
    }
}
```