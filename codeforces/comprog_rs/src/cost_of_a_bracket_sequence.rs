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
    let s = take_string();

    let mut ans = vec![0; n as usize];

    // find optimal arrangement
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        let a = s[i as usize];
        match a {
            '(' => {
                q.push_back(i);
                ans[i] = 2; // this
                ()
            }
            _ => {
                if !q.is_empty() {
                    let l = q.pop_back().unwrap();
                    ans[i] = 5; // this
                    ans[l] = 4; // last
                } else {
                    ans[i] = 3; // this
                }
                ()
            }
        }
    }
    let out = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}   {}", out, k);

    let mut forq = 0;
    let mut fivq = 0;

    // left to right, cull '('
    q.clear();
    for i in 0..n {
        let a = ans[i];
        match a {
            4 => {
                q.push_back(i);
                ()
            }
            3 => {
                if k > 0 && !q.is_empty() {
                    k -= 1;
                    ans[q.pop_front().unwrap()] = 1;
                    ans[i] = 0;
                    fivq -= 1;
                }
                ()
            }
            _ => (),
        }
    }
    q.clear();
    for i in (0..n).rev() {
        let a = ans[i];
        match a {
            5 => {
                q.push_back(i);
                ()
            }
            2 => {
                if k > 0 && !q.is_empty() {
                    k -= 1;
                    ans[q.pop_front().unwrap()] = 1;
                    ans[i] = 0;
                    forq -= 1;
                }
                ()
            }
            _ => (),
        }
    }
    let out = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}   {}", out, k);

    for &a in &ans {
        if a == 4 {
            forq += 1;
        }
        if a == 5 {
            fivq += 1;
        }
    }
    let target = if fivq >= forq { 4 } else { 5 };
    for i in {
        if target == 5 {
            (0..n).rev().collect::<Vec<usize>>()
        } else {
            (0..n).collect()
        }
    } {
        let a = ans[i];
        if a == target {
            if k > 0 {
                k -= 1;
                ans[i] = 1;
            }
        }
    }
    let out = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}      4:{} 5:{}", out,forq,fivq);
    // right to left, cull ')'
    let out = ans
        .iter()
        .map(|&x| if x > 1 { 0 } else { x })
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", out)
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
