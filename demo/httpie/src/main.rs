use clap::{ AppSettings, Clap };

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "zoomdong")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
  Get(Get),
  Post(Post),
}

struct Get {
  url: String,
}

struct Post {
  url: String,
  body: Vec<String>,
}

// Clap 提供了宏使得捕获 cli 输入简单
fn main () {
  let opts: Opts = Opts::parse();
  println!("{:?}", opts);
}