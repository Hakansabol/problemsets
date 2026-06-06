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

fn solve() {
    let n = take_int() as usize;
    let v = take_vector();
    let qq = take_int();

    let mut pv = vec![1000000; n];
    let mut lastidx = 1000000;
    for idx in (0..n - 1).rev() {
        if v[idx] != v[idx + 1] {
            lastidx = idx as i32 + 1;
        }
        pv[idx] = lastidx;
    }
    // println!("{:?}", pv);

    for _ in 0..qq {
        let t = take_vector();
        let (l, r) = (t[0] - 1, t[1] - 1);
        let get = pv[l as usize];
        if get <= r {
            println!("{} {}", l + 1, get + 1);
        } else {
            println!("{}", "-1 -1");
        }
    }
    println!("");
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
