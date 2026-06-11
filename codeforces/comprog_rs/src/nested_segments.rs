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

use std::collections::VecDeque;

use template::*;

pub fn main() {
    let n = take_int() as usize;
    let mut v = vec![];

    for i in 0..n {
        let tv = take_vector();
        v.push((tv[0], tv[1], i + 1)); // start, end, idx
    }

    v.sort_by_key(|x| x.0 * 2000000000 - x.1);
    // println!("{:?}", v);

    for i in 1..n {
        let l = v[i - 1];
        let r = v[i];
        if l.0 <= r.0 && l.1 >= r.1 {
            println!("{} {}", r.2, l.2);
            return;
        }
    }

    // let mut q: VecDeque<(i64, i64, usize)> = VecDeque::new();
    //
    // for a in v {
    //     let next = a.0;
    //     if !q.is_empty() {
    //         let last = q.back().unwrap();
    //         if last.0 >= a.0 && last.1 <= a.1 {
    //             println!("{} {}", last.2, a.2);
    //             return;
    //         }
    //     }
    //     q.push_back(a);
    // }
    println!("{}", "-1 -1");
}
