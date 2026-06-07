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

fn run_length_encoding<T: Eq + Default + Copy>(v: Vec<T>) -> Vec<(T, usize)> {
    let mut ans: Vec<(T, usize)> = vec![];
    let mut last_pos: usize = 0;
    for i in 1..v.len() {
        if v[i] != v[i - 1] {
            ans.push((v[i - 1], i - last_pos));
            last_pos = i;
        }
    }
    ans.push((v[v.len() - 1], v.len() - last_pos));
    ans
}
fn factmod(n: i64) -> i64 {
    let mut ans = 1;
    for i in 1..=n {
        ans = (ans * i) % 998244353;
    }
    ans
}

fn solve() {
    let s = take_string();

    let v: Vec<i64> = run_length_encoding(s)
        .into_iter()
        .map(|x| (x.1 - 1) as i64)
        .filter(|&x| x > 0)
        .collect();
    // println!("{:?}", v);

    let s: i64 = v.iter().sum();
    let c = v
        .into_iter()
        .fold(1i64, |acc, x| (acc * (x + 1)) % 998244353)
        * factmod(s)
        % 998244353;
    println!("{} {}", s, c);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
