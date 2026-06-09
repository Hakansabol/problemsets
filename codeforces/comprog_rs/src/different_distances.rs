// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i32>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use template::*;

fn solve() {
    let mut n = take_int();
    let mut enc = 0;

    let mut ans = vec![];
    if n % 2 == 1 {
        enc += 3;
        ans = vec![1, 1, 2, 1, 2, 3, 1, 3, 2, 2, 3, 3];
    }
    loop {
        if enc == n {
            break;
        }
        ans.push(enc + 1);
        ans.push(enc + 2);
        ans.push(enc + 2);
        ans.push(enc + 1);
        ans.push(enc + 2);
        ans.push(enc + 1);
        ans.push(enc + 1);
        ans.push(enc + 2);
        enc += 2;
    }
    let out = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
