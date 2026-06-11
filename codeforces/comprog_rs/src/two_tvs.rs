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
    let n = take_int();
    let mut events: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let tv = take_vector();
        events.push((tv[0], 1));
        events.push((tv[1], -1));
    }
    events.sort_by_key(|x| x.0 * 4 - x.1);

    let mut inuse = 0;
    for a in events {
        inuse += a.1;
        if inuse > 2 {
            println!("{}", "NO");
            return;
        }
    }
    println!("{}", "YES");
}

pub fn main() {
    solve();
}
