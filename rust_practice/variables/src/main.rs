use std::io;

fn main() {
    loop {
        let mut x1 = 1;
        let mut x2 = 1;
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("wrong input");
        let n: i32 = n.trim().parse().expect("Not number");
        for _i in 1..n {
            let x3 = x1 + x2;
            x1 = x2;
            x2 = x3;
        }
        println!("{}", x1);
    }
}
