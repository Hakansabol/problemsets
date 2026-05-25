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
    let n = take_int() as usize;
    let v = take_vector();

    let mut pivot = 0usize;
    let mut leftsum = 0i64;
    let mut ritesum = v.iter().sum::<i64>();
    let mut best = ritesum;
    for i in (1..=n) {
        let a = v[i - 1];
        leftsum += a.abs();
        ritesum -= a;
        if a > 0 {
            if leftsum + ritesum - a * 2 > best {
                pivot = i;
                best = leftsum + ritesum - a * 2;
            }
        }
    }

    if pivot == 0 {
        println!("{}", 0);
        println!("");
        return;
    }

    let mut parity: bool = false; // true: pos, false: neg
    let mut ans = vec![];
    for i in (1..pivot).rev() {
        let a = v[i - 1];
        if parity != (a > 0) {
            ans.push(i);
        }
        parity = a > 0
    }
    ans.push(pivot);

    println!("{}", ans.len());
    let out = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
