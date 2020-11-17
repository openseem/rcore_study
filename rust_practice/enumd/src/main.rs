#![allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor,
}
fn main() {
    let p = Message::Quit;
    let q2 = Message::Write(String::from("321323"));
    let p3 = Some(3);
    if let p3 = Some(3) {
    }
}
print!("{}", p3.);
