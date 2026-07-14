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

use std::collections::{VecDeque, vec_deque};

use template::*;

struct Tree {
    v: Vec<TreeNode>
}
struct TreeNode {
    val: i64,
    children: Vec<usize>,
}
impl Tree {
    fn insert(&mut self, idx: usize, parent: usize, val: i64) {
        self.v[idx] = TreeNode::new(val);
        self.v[parent].children.push(idx);
    }
    fn getChildren(&mut self, idx: usize) -> &Vec<usize> {
        &self.v[idx].children
    }
}
impl TreeNode {
    fn new(v: i64) -> Self {
        TreeNode {
            val: v,
            children: vec![],
        }
    }
}

fn solve() {
    let n = take_int() as usize;
    let p = take_vector();
    let a = take_vector();
    if n == 1 {
        println!("{}", "YES");
        return;
    }
    let mut leaves: Vec<Vec<i64>> = vec![vec![]; n];
    for i in 1..n {
        let parentidx = p[i - 1] - 1;
        leaves[parentidx as usize].push(i as i64 + 1);
    }
    println!("{:?}", leaves);

    let mut root = TreeNode::new(0);
    let mut nodes: Vec<TreeNode> = Vec::with_capacity(n);
    nodes[0] = root;
    let mut vd = VecDeque::new();
    vd.push_back(1);
    while !vd.is_empty() {
        let i: i64 = vd.pop_front().unwrap();
        for &nn in &leaves[(i - 1) as usize] {
            nodes[(nn - 1) as usize] = TreeNode::new(nn);
            (nodes[i as usize]).children.push(newn);
        }
    }
    for a in leaves {
        if a.len() > 2 {
            let mut acc = 0;
            for i in 0..a.len() {
                if a[i] < a[if i == 0 { a.len() - 1 } else { i - 1 }] {
                    acc += 1;
                }
                if acc > 1 {
                    println!("{}", "NO");
                    return;
                }
            }
        }
    }
    println!("{}", "YES");
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
