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

fn fib2(n: i64, cap: i64) -> (i64, i64) {
    let mut state = (0, 1);
    for _ in 1..n {
        let t = state.1;
        state.1 = state.0 + state.1;
        state.0 = t;
        if t > cap * 5 {
            return (i64::MAX, i64::MAX);
        }
    }
    state
}

fn solve() {
    let tv = take_vector();
    let (n, k) = (tv[0], tv[1]);

    let cnts = fib2(k, n);
    if cnts.0 == i64::MAX {
        println!("{}", 0);
        return;
    }
    // println!("{:?}", cnts);

    let mut ans = 0;
    for i in 0..=n {
        let v = i * cnts.0;
        if (n - v) >= 0 && (n - v) % cnts.1 == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
