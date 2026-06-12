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

/// zt: Frequency Map
/// Generate a frequency map from a vector
/// vec![1,1,3] becomes {1: 2, 3: 1}
use std::hash::Hash;
fn freqmap<T: Eq + Hash>(v: &Vec<T>) -> std::collections::HashMap<&T, i32> {
    let mut hm = std::collections::HashMap::new();
    for a in v {
        *hm.entry(a).or_insert(0) += 1;
    }
    hm
}
fn solve() {
    let tv = take_vector();
    let (n, k) = (tv[0], tv[1]);
    let v = take_vector();
    let mut v = freqmap(&v).into_iter().collect::<Vec<(_, _)>>();
    v.sort_by_key(|x| x.0);

    let mut last = -1;
    let mut ans = false;
    for a in v {
        if last > 0 {
            if a.0 - last <= k {
                ans = true;
            };
        }
        last = *a.0;

        if a.1 % 2 == 0 {
            ans = true;
        }
    }
    println!("{}", if ans { "YES" } else { "NO" });
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
