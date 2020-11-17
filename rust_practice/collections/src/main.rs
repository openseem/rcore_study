#![allow(unused)]

use std::collections::HashMap;
use std::convert::TryInto;

enum FirstType {
    Yuan(char),
    Fu(char),
}

fn main() {
    // let plot = vec![1, 1, 2, 3, 4, 1, 3, 2, 2, 3, 2];
    // println!(
    //     "mean:{} mid:{} mode:{}",
    //     get_mean(&plot),
    //     get_mid(&plot),
    //     get_mode(&plot)
    // );

    let mut str = String::from("first");
    let first = get_first(&str);
    match first {
        FirstType::Yuan(f) => {
            let mut str = &mut str;
            str.push_str("-hay");
        }
        FirstType::Fu(f) => {
            str.remove(0);
            str.push('-');
            str.push(f);
            str.push_str("ay");
        }
    }
    println!("{}", &str);
}

fn get_first(str: &str) -> FirstType {
    let mut ans: Option<FirstType> = None;
    for i in str.chars() {
        if i == 'a' || i == 'e' || i == 'o' || i == 'u' {
            ans = Option::Some(FirstType::Yuan(i));
        } else {
            ans = Option::Some(FirstType::Fu(i));
        }
        break;
    }
    match ans {
        Option::Some(s) => s,
        _ => FirstType::Yuan('2'),
    }
}

fn get_mean(vec: &Vec<i32>) -> f64 {
    let mut ans = 0;
    let mut num = 0;
    for i in vec {
        ans += *i;
        num += 1;
    }
    (ans / num).into()
}

fn get_mid(vec: &Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut len = 0;
    for i in vec {
        len += 1;
        if len == (vec.len() / 2).try_into().unwrap() {
            ans = *i;
        }
    }
    return ans;
}

fn get_mode(vec: &Vec<i32>) -> i32 {
    let mut mp = HashMap::new();
    let mut max = 0;
    mp.insert(1, 0);
    for i in vec {
        let count = mp.entry(*i).or_insert(0);
        *count += 1;
        if *count > max {
            max = *count;
        }
    }
    max
}
