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

    // get the bonus
    let mut mins = vec![];

    // get the value if no removal is made
    let mut lowest = n as i64 + 1;
    let mut def = 0i64;
    for i in (0..n).rev() {
        let a = v[i];
        def += std::cmp::max(0, a - lowest);
        if a < lowest {
            lowest = a;
            mins.push(i as i64);
        }
    }
    mins.push(-1);

    let mut magic = 0;
    for i in (0..mins.len() - 1) {
        magic = std::cmp::max(magic, mins[i] - mins[i + 1] - 1);
    }

    println!("{}", magic + def);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
