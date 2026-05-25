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

use std::collections::HashSet;

use template::*;

fn solve() {
    let n = take_int() as usize;
    let v = take_vector();

    let m = v.iter().cloned().max().unwrap();
    let hm: HashSet<i64> = v.into_iter().collect();
    let mut go = true;
    let mut acc = 0i64;
    let mut ans = 0i64;
    for i in 1..n {
        if go && hm.get(&(i as i64)).is_some() {
            acc += 1;
        } else {
            go = false;
        }
        ans += m + acc;
    }
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
