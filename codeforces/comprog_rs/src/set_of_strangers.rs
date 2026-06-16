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

use std::collections::{HashMap, HashSet, VecDeque};

use template::*;

fn solve() {
    let tv = take_vector();
    let (rc, cc) = (tv[0] as usize, tv[1] as usize);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut next: VecDeque<(usize, usize)> = VecDeque::new();

    let mut colcnts: HashMap<i32, i32> = HashMap::new();

    let mut g: Vec<Vec<i32>> = vec![];
    for r in 0..rc {
        g.push(take_vector());
    }

    for r in 0..rc {
        for c in 0..cc {
            if visited.contains(&(r, c)) {
                continue;
            }

            let a = g[r][c];
            let mut fillc = 1;
            next.push_back((r, c));
            visited.insert((r, c));
            while !next.is_empty() {
                let (r, c) = next.pop_front().unwrap();
                for (rd, cd) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (rn, cn) = (r as i32 + rd, c as i32 + cd);
                    if rn >= 0 && rn < rc as i32 && cn >= 0 && cn < cc as i32 {
                        if !visited.contains(&(rn as usize, cn as usize))
                            && g[rn as usize][cn as usize] == a
                        {
                            next.push_back((rn as usize, cn as usize));
                            visited.insert((rn as usize, cn as usize));
                            fillc += 1;
                        }
                    }
                }
            }
            *colcnts.entry(a).or_insert(0) = std::cmp::max(*colcnts.entry(a).or_insert(0), fillc);
        }
    }

    let mut vans = colcnts
        .values()
        .into_iter()
        .map(|x| std::cmp::min(*x, 2))
        .collect::<Vec<i32>>();
    // println!("{:?}", vans);
    vans.sort();
    let ans = vans.iter().sum::<i32>() - vans[vans.len() - 1];
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
