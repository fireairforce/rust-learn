// 命令行参数解析
use clap::Parser;
// 校验字段是否合法
use anyhow:: { Result, anyhow };
use reqwest::{
  header,
  Url,
  Client,
  Response
};
use std::{ 
  str::FromStr,
  collections::HashMap
};
// 处理 cli 输出高亮
use syntect::{
  easy::HighlightLines,
  highlighting::{Style, ThemeSet},
  parsing::SyntaxSet,
  util::{as_24_bit_terminal_escaped, LinesWithEndings}
};

use colored::Colorize;
use mime::Mime;

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

// 这里要加上 PartialEq 否则测试会跑不过去(派生的例子)
// 参考文档: https://rustwiki.org/zh-CN/rust-by-example/trait/derive.html
#[derive(Debug, PartialEq)]
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

// 处理 get 子命令
async fn get(clinet: Client, args: &Get) -> Result<()> {
  let resp = clinet.get(&args.url).send().await?;
  // println!("{:?}", resp.text().await?);
  Ok(print_resp(resp).await?)
}

// 处理 post 子命令
async fn post(clinet: Client, args: &Post) -> Result<()> {
  let mut body = HashMap::new();
  for pair in args.body.iter() {
    body.insert(&pair.k, &pair.v);
  }
  let resp = clinet.post(&args.url).json(&body).send().await?;
  // println!("{:?}", resp.text().await?);
  Ok(print_resp(resp).await?)
}

// 打印服务器 version + statuc code
fn print_status(resp: &Response) {
  let status = format!("{:?} {}", resp.version(),resp.status()).blue();
  println!("{}\n", status);
}

// 打印 headers
fn print_headers(resp: &Response) {
  for (name, value) in resp.headers() {
    println!("{}: {:?}", name.to_string().green(), value);
  }

  println!();
}

// 打印 http body
fn print_body(m: Option<Mime>, body: &str) {
  match m {
    // "application/json", pretty print
    Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
    Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),

    // 其它 mime type 直接打印出去
    _ => println!("{}", body),
  }
}

// 打印响应
async fn print_resp(resp: Response) -> Result<()> {
  print_status(&resp);
  print_headers(&resp);
  let mime = get_content_type(&resp);
  let body = resp.text().await?;
  print_body(mime, &body);
  Ok(())
}

fn get_content_type(resp: &Response) -> Option<Mime> {
  resp.headers()
      .get(header::CONTENT_TYPE)
      .map(|v| v.to_str().unwrap().parse().unwrap())
}

// Clap 提供了宏使得捕获 cli 输入简单
#[tokio::main]
async fn main () -> Result<()> {
  let opts: Opts = Opts::parse();
  // http 客户端生成
  let mut headers = header::HeaderMap::new();
  // 为 http 客户端补一些缺省的 http 头
  headers.insert("X-POWERED-BY", "Rust".parse()?);
  headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
  let client = reqwest::Client::builder().default_headers(headers).build()?;
  let result = match opts.subcmd {
    SubCommand::Get(ref args) => get(client, args).await?,
    SubCommand::Post(ref args) => post(client, args).await?,
  };

  Ok(result)
}

fn print_syntect(s: &str, ext: &str) {
  // load these once at the start of program
  let ps = SyntaxSet::load_defaults_newlines();
  let ts = ThemeSet::load_defaults();
  let syntax = ps.find_syntax_by_extension(ext).unwrap();
  let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
  for line in LinesWithEndings::from(s) {
    let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
    println!("{}", escaped);
  }
}

// 编写一下测试, cfg test 表示整个 mod tests 只在 cargo test 的时候才编译
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_url_works() {
    assert!(parse_url("abc").is_err());
    assert!(parse_url("http://abc.xyz").is_ok());
    assert!(parse_url("https://httpbin.org/post").is_ok());
  }

  #[test]
  fn parse_kv_pair_works() {
    assert!(parse_kv_pair("a").is_err());
    assert_eq!( parse_kv_pair("a=1").unwrap(), KvPair { k: "a".into(), v: "1".into() } );
    assert_eq!( parse_kv_pair("b=").unwrap(), KvPair { k: "b".into(), v: "".into() } );
  }
}