use clap::Parser;
use anyhow:: { Result, anyhow };
use reqwest::Url;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "zoomdong")]
struct Opts {
  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
  Get(Get),
  Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
  #[clap(parse(try_from_str = parse_url))]
  url: String,
}

#[derive(Parser, Debug)]
struct Post {
  #[clap(parse(try_from_str = parse_url))]
  url: String,
  #[clap(parse(try_from_str = parse_kv_pair))]
  body: Vec<KvPair>,
}

#[derive(Debug)]
struct KvPair {
  k: String,
  v: String,
}

// 来自官方 std 的一个 trait FromStr
impl FromStr for KvPair {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // 使用 = 进行 split, 生成一个叫 split 的迭代器
    let mut split = s.split("=");
    let err = || anyhow!(format!("Failed to parse {}", s));
    Ok(Self {
      // 迭代器里面返回的第一个结果 key, 返回 Some(T) / Node
      // 第一个结果，使用 ? 来进行错误处理
      k: (split.next().ok_or_else(err)?).to_string(),
      // 迭代器的第二个结果为 value
      v: (split.next().ok_or_else(err)?).to_string(),
    })
  }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
  Ok(s.parse()?)
}

fn parse_url(s: &str) -> Result<String> {
  // check 一下 url 是否合法
  let _url: Url = s.parse()?;

  Ok(s.into())
}

// Clap 提供了宏使得捕获 cli 输入简单
fn main () {
  let opts: Opts = Opts::parse();

  println!("{:?}", opts);
}