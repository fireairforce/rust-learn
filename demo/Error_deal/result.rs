use std::fs::File;

fn main() {
  let f = File::open("Readme.md");

  let f = match f {
    Ok(file) => file,
    Err(err) => {
      panic!("Problem opening file: {:?}", err);
    }
  }
}