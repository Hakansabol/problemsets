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

use std::{collections::HashSet, ops::Add};

use template::*;

fn solve() {
    let n = take_int() as usize;
    let v = take_vector();

    let mut wins: HashSet<i64> = HashSet::new();
    wins.insert(0);
    let mut oddacc = 0i64;
    let mut eveacc = 0i64;

    for i in 0..n {
        *if i % 2 == 0 { &mut eveacc } else { &mut oddacc } += v[i];
        let difft = eveacc - oddacc;

        if wins.contains(&difft) {
            println!("{}", "YES");
            return;
        }
        wins.insert(difft);
    }
    println!("{}", "NO");
}

// 4 1 3 2
// 40417173

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
