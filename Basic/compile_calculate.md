## 编译期计算

最早由 lisp / cpp 支持。

### rust 支持的两种方式
- 进程宏 + Build 脚本(build.rs)
- 类似于 Cpp 中的 constexpr 的 CTFE（常量表达式) 功能

### rust 的 CTFE

- 常量函数

```rust
// = 后面要求是个常量表达式，能直接用于计算
const X: T = ...;
```

```rs
fn main() {
  let an = (42,).0;
  const AN: i32 = an; // error: 赋值错了


  // fixed: 
  const AN: i32 = (42,).0;
}
```

- 常量上下文

1. 常量值初始化位置
2. 静态数组的长度表达式: [T;N]
3. 重复的长度表达式: 类似于: [0; 10]
4. 静态变量、枚举判别式初始化位置

常量传播和编译器计算是不用的:
- 编译器优化
- 并不能改变程序任何行为，开发者无感知
- 编译器计算则是编译时之行的代码，必须知道结果才能继续执行

```rs
const X: u32 = 3 + 4; // CTFE
ler x: uew = 3 + 4;
```

- 常量函数

支持递归和嵌入式

```rs
const fn fib(n: u128) -> u128 {
  const fn helper(n: u128, a: u128, b: u128, i: u128) -> u128 {
    if i <= n {
      helper(n, b, a + b, i + 1)
    } else {
      b
    }
  }
  helper(n, 1, 1, 2)
}

const X: u128 = fib(10);

fn main() {
  println!("{}", X);
}
```

以上过程都是在编译期完整的，对于性能有显著的提升


rust 会提示使用 loop 来替代 `while true`

rust 编译器不会识别 while true,因为这是一种特殊的情况

1. 要考虑 while(constexpr == true) 的情况
2. 使用 #[allow(while_true)] 属性在某些情况下允许使用 while true

算得上是 rust 在语言设计上的一种缺陷吧。

- 常量泛型

Rust 中的静态数组一直以来都属于 二等公民，并不方便使用

```rs
let arr : [3; i32] = [1,2,3];
let arr : [5; i32] = [1,2,3,4,5];
```

Rust 里面 这里第一个数组和第二个数组是不同的类型。因此需要使用常量泛型来解决这些问题:

```rs
#![feature](min_const_generics)

use core:mem::MaybeUninit;

pub struct ArrayVec<T, const N: usize> {
  items: [MaybeUninit<T>; N],
  length: usize,
}
```