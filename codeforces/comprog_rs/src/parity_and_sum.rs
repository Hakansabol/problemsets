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
    let n = take_int();
    let v = take_vector();

    let mut odds = vec![];
    let mut eves = vec![];

    for a in &v {
        (if a % 2 == 0 { &mut eves } else { &mut odds }).push(*a);
    }

    // instant win
    if std::cmp::min(odds.len(), eves.len()) == 0 {
        println!("{}", 0);
        return;
    }
    eves.sort();

    let mut ans = eves.len();

    let mut oddacc = *odds.iter().max().unwrap();
    for a in eves {
        if a > oddacc {
            ans += 1;
            break;
        }
        oddacc += a;
    }

    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
