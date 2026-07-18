// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
// Functions prefixed with "/// zt..." are snippets and can be read in my dotfiles: https://github.com/Hakansabol/dotfiles/blob/main/nvim/lua/snips/snips_rust.lua
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

    let v = take_vector();
    let mut v = v
        .iter()
        .enumerate()
        .map(|x| (x.0, x.1.clone()))
        .collect::<Vec<_>>();
    v.sort_by_key(|x| x.1);
    let mut state: (i64, i64) = (-1, -1);
    let mut nmap = (0..n as i64).collect::<Vec<i64>>();
    for i in 0..n {
        if i > 0 && v[i].1 == state.1 {
            v[i].1 = (state.0 as i64) as i64;
        } else {
            state = (i as i64, v[i].1);
            v[i].1 = i as i64;
        }
    }
    v.sort_by_key(|x| x.0);
    let mut v = v.iter().map(|x| x.1).collect::<Vec<_>>();
    // println!("{:?}", v);
    let mut ans = 0i64;
    for i in 0..n {
        let a = v[i];
        let target = nmap[a as usize];
        let src = i as i64;
        nmap[a as usize] += 1;
        let temp = src ^ target;
        let lz = temp.leading_zeros();
        let o = temp >> (63 - lz) << (63 - lz);
        // println!("{} {}", temp, o);
        ans = std::cmp::max(ans, o);
    }

    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}

// in each block with a new bit, it is best to only swap within the block.
// 1 2 3 4 5 7 6
// 2 because 7 and 6 can swap with each other
// swapping between blocks necessitates k >= [value of the block]
// 7 0 2 3 4 5 6 1
// 4 because 0:4 4:7 0:4
// basically each element in the wrong place sets k to max(k, [value of most significant bit in
// idx^targidx])
// duplicates?
// 1 3 2 7 5 6 3
// 1 1 1 1 1 1 1
