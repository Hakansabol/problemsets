// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i64>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use template::*;

fn solve() {
    let n = take_int() as usize;
    let mut v = take_vector();

    v.sort();

    let mut y = v[n - 1];
    let mut x = v[n - 2];
    for i in (0..n - 2).rev() {
        let need = y % x;
        if v[i] != need {
            println!("{}", "-1");
            return;
        }
        y = x;
        x = need;
    }
    println!("{} {}", v[n - 1], v[n - 2]);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
