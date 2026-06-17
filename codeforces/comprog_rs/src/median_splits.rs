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
    let (n, k) = (tv[0] as usize, tv[1]);
    let v = take_vector();
    let mut v = v
        .into_iter()
        .map(|x| if x <= k { 1 } else { -1 })
        .collect::<Vec<i64>>();

    let (mut ds, mut rs) = (i64::MAX, i64::MAX);

    // try case !!. and .!!
    // then try case !.!
    for dir in 0..2 {
        let (mut nc, mut pc) = (0, 0);
        let mut grace = 0;
        let mut successes = 0;
        for i in 0..n-1 {
            let a = v[i] == 1;
            if a {
                pc += 1
            } else {
                if grace > 0 { grace -= 1 } else { nc += 1 }
            };
            if pc >= nc && pc > 0 {
                grace = pc - nc;
                pc = 0;
                nc = 0;
                successes += 1;
                match dir {
                    0 => {
                        if ds == i64::MAX {
                            ds = i as i64
                        }
                    }
                    _ => {
                        if rs == i64::MAX {
                            rs = i as i64
                        }
                    }
                }
            }
            // println!("{} {} {} {}", pc, nc, grace, successes);
        }
        if successes >= 2 
            || (ds < i64::MAX && rs < i64::MAX && ds + rs < n as i64) 
                {
            println!("{}", "YES");
            return;
        }
        v.reverse();
    }

    println!("{}", "NO");
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
