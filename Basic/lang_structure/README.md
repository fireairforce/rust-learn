## 语法结构

### 语意化版本

同 npm 的 semver 版本.

x.y.z规则来开发。

### 发行版本

#### 版次

通过在 `Cargo.toml` 中配置 `edition` 字段来配置版次.

- 2015 Edition
- 2018 Edition
- 2021 Edition

每隔三年会对 rust 三年的特性做个总结.

## Rust 词法结构

### 编译器

rustc.

### 关键字

- 严格关键字(不能用于其他用途)
- 保留字(不代表未来一定会使用)
- 弱关键字(特殊场景中使用)


### 标识符

rust 目前仅仅支持以 ascii 字符串开头的变量名作为标识符号.

目前有个 rfc 在讨论支持非 ascii 开头的标识符。特定的语言或者场景可以转换实现。

```rust
fn main() {
  let thinking = "thinking";
  let thinking123_ = "thinking 123";

  // err 不能以数字字符串开头
  let 312_fuck = "fucking";

  // ok
  let _321_fuck = "fucking";

  // err
  let 🤔 = "hhh";
}
```

### 注释

rust 支持多种注释

```rust
//! - 模块文档注释，放在模块头部
//!! - 和上面注释位于同一行

//! 换行

/*! - 注释多段代码 - */
```



