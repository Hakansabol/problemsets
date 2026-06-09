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

use std::collections::HashSet;

use template::*;

fn solve() {
    let tv = take_vector();
    let (n, k) = (tv[0], tv[1]);
    let v = take_vector();

    let mut hs: HashSet<i32> = (1..=n).collect();

    for &a in &v {
        hs.remove(&a);
    }

    let hs: Vec<i32> = hs.into_iter().collect();

    let mut within: HashSet<i32> = HashSet::new();
    let vdd = v
        .iter()
        .filter(|&x| {
            let ret = !within.contains(x);
            within.insert(*x);
            ret
        })
        .map(|&x| x)
        .collect::<Vec<i32>>();

    let x = match hs.len() {
        0 => (0..k).map(|x| v[(x % n) as usize]).collect::<Vec<i32>>(),
        1 => (0..k)
            .map(|x| match x % 3 {
                0 => hs[0],
                1 => vdd[(vdd.len() - 2) as usize],
                _ => vdd[(vdd.len() - 1) as usize],
            })
            .collect::<Vec<i32>>(),
        2 => (0..k)
            .map(|x| match x % 3 {
                0 => hs[0],
                1 => hs[1],
                _ => vdd[(vdd.len() - 1) as usize],
            })
            .collect::<Vec<i32>>(),
        _ => (0..k).map(|x| hs[(x % 3) as usize]).collect::<Vec<i32>>(),
    };

    println!(
        "{}",
        x.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
