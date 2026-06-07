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

    let mut out = vec![0; n];
    for i in 0..n {
        let mut lidx = (i + n - 1) % n;
        let mut ridx = i;
        let mut ans = 0;
        let mut addl = 0;
        while lidx != ridx {
            let l = v[lidx];
            let r = v[ridx];
            if l < r {
                addl = std::cmp::max(addl, l);
                ans += addl;
                lidx = (lidx + n - 1) % n;
            } else {
                addl = std::cmp::max(addl, r);
                ans += addl;
                ridx = (ridx + 1) % n;
            }
        }
        out[i] = ans;
    }
    let out = out .iter() .map(|x| x.to_string()) .collect::<Vec<String>>() .join(" ");
    println!("{}", out);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
