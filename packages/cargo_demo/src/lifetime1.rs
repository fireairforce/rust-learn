fn main() {
    let s1 = String::from("wd");
    let s2 = String::from("wd2");

    let res = max(&s1, &s2);
    println!("bigger one: {}", res);
}

// 这里需要添加上生命周期的标注
// 不然 compiler 无法确定 函数的参数之间生命周期的关系，函数返回值之间生命周期的关系
fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
