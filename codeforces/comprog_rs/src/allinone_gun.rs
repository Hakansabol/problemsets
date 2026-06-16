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
    let tv = take_vector();
    let (n, mut h, k) = (tv[0], tv[1], tv[2]);
    let v = take_vector();

    // full cycles, unaffected by the swap
    let rounddam: i64 = v.iter().sum();
    let rotcnt = (h - 1) / rounddam;
    let ans = rotcnt * (k + n);
    h -= rounddam * rotcnt;

    // reverse max prefix
    let mut rms = v
        .iter()
        .rev()
        .scan(0, |acc, x| {
            let v = (*acc).clone();
            *acc = std::cmp::max(*acc, *x);
            Some(v)
        })
        .collect::<Vec<_>>();
    rms.reverse();
    // println!("{:?}", rms);

    // at any point, bail by taking the max value remaining.
    let mut sum = 0;
    let mut minl = i64::MAX - 5000000000;
    for i in 0..n as usize {
        let a = v[i];
        sum += a;
        minl = std::cmp::min(minl, a);
        if sum + std::cmp::max(0, rms[i] - minl) >= h {
            println!("{}", i + 1 + ans as usize);
            return;
        }
    }
    panic!();
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
