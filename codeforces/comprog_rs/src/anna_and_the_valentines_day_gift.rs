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

fn val(n: Vec<char>) -> (i32, i32) {
    let mut cnt = 0;
    for i in (0..n.len()).rev() {
        if n[i] == '0' {
            cnt += 1;
        } else {
            return (n.len() as i32, cnt);
        }
    }
    (n.len() as i32, cnt)
}

fn solve() {
    let tv = take_vector();
    let (n, m) = (tv[0], tv[1]);
    let mut v: Vec<(i32, i32)> = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.chars().collect())
        .map(|x| val(x))
        .collect();

    v.sort_by_key(|x| x.1);

    let mut mv = 1;
    let mut ans = 0;
    for idx in (0..v.len()).rev() {
        ans += v[idx].0;
        ans -= v[idx].1 * mv;
        mv = mv ^ 1;
    }

    println!("{}", if ans > m { "Sasha" } else { "Anna" });
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
