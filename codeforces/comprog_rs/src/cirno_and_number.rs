// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i128>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use std::collections::{VecDeque, vec_deque};

use template::*;

fn solve() {
    let v2 = take_vector();
    let v = take_vector();
    let (a, n) = (v2[0], v2[1]);

    let mut ok = vec![false; 10];
    for &a in &v {
        ok[a as usize] = true;
    }

    let mut l10 = 0;
    let mut cn = a - 1;
    while cn > 0 {
        l10 += 1;
        cn /= 10;
    }

    let mut space = vec![0i128; 1usize];
    let mut space_size = 1;
    for dig in (0..=l10 + 1).rev() {
        let mut nsp = vec![];
        let mul: i128 = 10i128.pow(dig as u32);
        for &a in &space {
            for &b in &v {
                nsp.push(a + b * mul);
            }
            if a == 0 && !ok[0] {
                nsp.push(0);
            }
        }
        nsp.sort_by_key(|x| (x - a).abs());
        // println!("{:?}", nsp);
        space = nsp[0..std::cmp::min(1001, nsp.len())].to_vec();
    }
    println!("{}", (space[0] - a).abs());
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
