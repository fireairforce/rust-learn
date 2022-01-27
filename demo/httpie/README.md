## Doc

1. 命令行解析
2. 处理参数发送 http 请求，获得响应
3. 对用户友好的输出响应

## 使用

本地开发调试可以使用 `cargo run xxx` 来:

```bash
cargo build --quiet && target/debug/httpie-zoomdong post https://httpbin.org/post greeting=hola name=zoomdong
```

使用编译之后的包，可以先执行:

```bash
cargo build --release
```

然后执行对应编译目录下面的可执行文件即可:

```bash
./target/release/httpie-zoomdong post https://httpbin.org/post greeting=hola name=zoomdong
```