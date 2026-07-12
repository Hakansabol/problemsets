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
    let n = take_int() as usize;
    let mut a = take_vector()
        .into_iter()
        .map(|x| x as usize)
        .collect::<Vec<usize>>(); // 1 indexed
    let mut b = take_vector()
        .into_iter()
        .map(|x| if x == -1 { 0usize } else { x as usize })
        .collect::<Vec<usize>>(); // 1 indexed
    a.insert(0, 0);
    b.insert(0, 0);
    let mut used = vec![true; n + 1]; // 1 indexed

    for &val in &b {
        if val == 0 {
            continue;
        }
        if !used[val] {
            println!("{}", "NO");
            return;
        } // REJECT
        used[val] = false;
    }

    // first, follow paths from b to fill c.
    for i in 1..=n {
        let mut aval = a[i] as usize; // 2
        let mut bval = b[i] as usize; // 2
        if bval == 0 {
            continue;
        }

        let s = bval;

        while b[aval] == 0 || b[aval] == a[bval] {
            // if b[aval] != 0 && b[aval] != a[bval] { println!("{}", "NO"); return; } // REJECT
            b[aval] = a[bval];

            // println!("{:?}", b);
            if b[aval] == s { break;}
            if !used[a[bval]] {
                println!("{}", "NO");
                return;
            } // REJECT
            used[a[bval]] = false;

            // println!("{} {}", aval,bval);
            // CHANGE I TO NEW VAL
            // i == c[aval]
            bval = b[aval] as usize;
            aval = a[aval] as usize;
        }
    }
    // then fill the rest with the remaining numbers in ascending order
    let mut pnt = 1;
    for i in 1..=n {
        if b[i] == 0 {
            while !used[pnt] {
                pnt += 1;
            }
            b[i] = pnt;
            pnt += 1;
        }
    }
    for i in 1..=n {
        if b[a[i]] != a[b[i]] {
            println!("{}", "NO");
            return; // REJECT
        }
    }
    // println!("{:?}", b);
    println!("{}", "YES");
    let out = b
        .iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
