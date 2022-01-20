fn main () {
  // 使用 prost 来使用和编译 prost
  // 将 abi.proto 编译到 src/pb 目录下面
  prost_build::Config::new()
    .out_dir("src/pb")
    .compile_protos(&["abi.proto"], &["."])
    .unwrap()
}

