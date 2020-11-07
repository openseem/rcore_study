fn main() {
    let mut str = String::from("hello world");
    let s = &mut str;
    s.push_str("string");
    // str.push_str("string");
    pt(str);
    pt(str);
}

fn pt(s: String) {
    println!("{}", s);
}
