use clap::Parser;

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
  url: String,
}

#[derive(Parser, Debug)]
struct Post {
  url: String,
  body: Vec<String>,
}

// Clap 提供了宏使得捕获 cli 输入简单
fn main () {
  let opts: Opts = Opts::parse();
  println!("{:?}", opts);
}