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

use std::hash::Hash;

use template::*;

/// zt: Frequency Map
/// Generate a frequency map from a vector
/// vec![1,1,3] becomes {1: 2, 3: 1}
fn freqmap<T: Eq + Hash>(v: &Vec<T>) -> std::collections::HashMap<&T, i32> {
    let mut hm = std::collections::HashMap::new();
    for a in v {
        *hm.entry(a).or_insert(0) += 1;
    }
    hm
}
fn solve() {
    let n = take_int() as usize;
    let v = take_vector();

    let mut mmm = freqmap(&v);
    for a in mmm.values_mut() {
        *a -= 1;
    }
    if *mmm.values().max().unwrap() <= 0 {
        println!("{}", 0);
        return;
    }

    let mut l = 0;
    let mut r = 0;
    let mut ans = i64::MAX;
    while r < n {
        *mmm.entry(&v[r]).or_insert(0) -= 1;
        r += 1;
        while r > l && *mmm.values().max().unwrap() <= 0 {
            ans = std::cmp::min(ans, (r - l) as i64);
            *mmm.entry(&v[l]).or_insert(0) += 1;
            l += 1;
        }
    }
    if ans == i64::MAX {
        println!("{}", 0);
    } else {
        println!("{}", ans);
    }
}

pub fn main() {
    solve();
}
