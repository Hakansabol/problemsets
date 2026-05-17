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
    let v = take_vector();
    let (n, m) = (v[0], v[1]);
    let a = take_vector();
    let b = take_vector();

    let mut odds: Vec<i64> = a.clone().into_iter().step_by(2).collect();
    let mut a = a.into_iter();
    a.next();
    let mut even: Vec<i64> = a.clone().into_iter().step_by(2).collect();

    let (mut oc, mut ec) = (0, 0);
    for a in b {
        if a % 2 == 0 { ec += 1 } else { oc += 1 }
    }

    odds.sort();
    even.sort();
    // let out = odds
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<String>>()
    //     .join(" ");
    // println!("{}", out);
    // let out = even
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<String>>()
    //     .join(" ");
    // println!("{}", out);

    let mut ans = 0i64;

    for i in odds.len() - std::cmp::min(odds.len(), oc)..odds.len() {
        if odds[i] > 0 || i == odds.len() - 1 {
            ans -= odds[i];
        }
    }
    for i in even.len() - std::cmp::min(even.len(), ec)..even.len() {
        if even[i] > 0 || i == even.len() - 1 {
            ans -= even[i];
        }
    }
    ans += odds.iter().sum::<i64>();
    ans += even.iter().sum::<i64>();
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
