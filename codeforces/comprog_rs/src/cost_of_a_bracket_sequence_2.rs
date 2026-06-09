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

use std::collections::VecDeque;

use template::*;

fn solve() {
    let tv = take_vector();
    let (n, mut k) = (tv[0] as usize, tv[1]);
    let mut s = take_string()
        .iter()
        .map(|&x| if x == '(' { 2 } else { 3 })
        .collect::<Vec<i32>>();

    let mut ans = vec![0; n];
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut q2: VecDeque<usize> = VecDeque::new();
    let mut balance = 0;
    for i in 0..n {
        let a = s[i];
        if a == 2 {
            q.push_back(i);
            balance += 1;
        }
        if a == 3 {
            q2.push_back(i);
            balance -= 1;
            if balance < 0 {
                while k > 0 && !q.is_empty() {
                    k -= 1;
                    ans[q.pop_front().unwrap()] = 1;
                }
                q.clear();
                q2.clear();
                balance = 0;
            }
        }
    }

    while k > 0 && !q2.is_empty() {
        k -= 1;
        ans[q2.pop_back().unwrap()] = 1;
    }

    let out = ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    println!("{}", out)
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
