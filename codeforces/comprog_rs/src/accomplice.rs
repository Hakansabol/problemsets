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

fn main() {
    let tv = take_vector();
    let (n, d) = (tv[0] as usize, tv[1]);
    let mut v = vec![];
    for _ in 0..n {
        let tv = take_vector();
        v.push((tv[0], tv[1]))
    }

    let mut modarr = vec![(0, 0); 2345678];
    let mut su: i64 = 0;
    for &a in &v {
        modarr[a.1 as usize].1 += 1;
        modarr[(a.0+d) as usize].0 += 1;
    }
    // println!("{:?}", &modarr[0..50]);

    let mut ans: i64 = 0;
    for i in 1..modarr.len() {
        let a = modarr[i];
        su += a.0;
        if su > 1 {
            let ch = ((su * (su - 1)) / 2);
            if ch > 0 {
                // println!("{}:{}",i, ch);
            }
            ans += ch;
        }
        su -= a.1;
    }

    println!("{}", ans);
}
