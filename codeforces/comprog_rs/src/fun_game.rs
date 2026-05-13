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
    let n = take_int();
    let s1 = take_string();
    let s2 = take_string();

    for i in 0..n as usize {
        if s1[i] == '1' {
            println!("{}", "YES");
            return;
        }
        if s2[i] == '1' {
            println!("{}", "NO");
            return;
        }
    }
    println!("{}", "YES");
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
