struct User {
  username: String;
  email: String;
  sign_in_count: u64;
  active: bool;
}

let mut user1 = User {
  email: String::from("wudong@hrsoft.net"),
  username: String::from("wudong"),
  active: true,
  sign_in_count: 1
}


user1.email = String::from("wudong.zelda@bbbxxx");

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    
  }
}