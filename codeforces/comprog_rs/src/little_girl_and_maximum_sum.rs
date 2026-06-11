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
    let (n, q) = (tv[0] as usize, tv[1]);

    let mut v = take_vector();
    v.sort_by_key(|x| -x);

    let mut pm = vec![0; n];
    pm.push(0);

    for _ in 0..q {
        let datum = take_vector();
        let (a, b) = (datum[0] - 1, datum[1]);

        pm[a as usize] += 1;
        pm[b as usize] -= 1
    }

    pm = pm
        .into_iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    pm.pop();

    pm.sort_by_key(|x| -x);
    let mut ans = 0;
    for i in 0..n {
        ans += pm[i] * v[i];
    }
    println!("{}", ans);
}

pub fn main() {
    solve();
}
