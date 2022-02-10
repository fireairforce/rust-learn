## thumbor

图片服务器

## 使用

可以结合 httpie-zoomdong 来调试

```bash
# 项目服务先 run 起来
cargo run 

# 去 httpie 那边请求一下
./target/release/httpie-zoomdong get "http://localhost:3000/image/CgoKCAjYBBCgBiADCgY6BAgUEBQKBDICCAM/https%3A%2F%2Fimages%2Epexels%2Ecom%2Fphotos%2F2470905%2Fpexels%2Dphoto%2D2470905%2Ejpeg%3Fauto%3Dcompress%26cs%3Dtinysrgb%26dpr%3D2%26h%3D750%26w%3D1260"
```