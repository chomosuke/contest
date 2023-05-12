#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
};

fn next_nodes(node: u64) -> Vec<u64> {
    let mut res = HashSet::new();
    let sq = (node as f64).sqrt() as u64;
    for x in 1..=sq {
        res.insert(node + node / x);
        res.insert(node + x);
    }
    res.into_iter().collect::<Vec<_>>()
}

type Value = u64;
type Weight = u64;

pub fn knapsack_01(items: &[(Value, Weight)], max_w: u64) -> Value {
    _knapsack_01(items, 0, max_w, &mut HashMap::new())
}

fn _knapsack_01(
    items: &[(Value, Weight)],
    processed: usize,
    rem_w: Weight,
    mem: &mut HashMap<(usize, Weight), Value>,
) -> Value {
    if processed >= items.len() {
        return 0;
    }
    let key = (processed, rem_w);
    if mem.get(&key).is_none() {
        let item = items[processed];
        let max_v = if rem_w >= item.1 {
            max(
                _knapsack_01(items, processed + 1, rem_w, mem),
                _knapsack_01(items, processed + 1, rem_w - item.1, mem) + item.0,
            )
        } else {
            _knapsack_01(items, processed + 1, rem_w, mem)
        };
        mem.insert(key, max_v);
    }
    mem[&key]
}

fn main() {
    let mut sc = Scanner::new();
    let test_case = sc.next::<usize>();
    for _ in 0..test_case {
        let n = sc.next::<usize>();
        let k = sc.next::<usize>();
        let mut bs = Vec::with_capacity(n);
        for _ in 0..n {
            bs.push(sc.next::<u64>());
        }
        let mut cs = Vec::with_capacity(n);
        for _ in 0..n {
            cs.push(sc.next::<u64>());
        }

        // BFS all ks
        let mut ops = HashMap::<u64, u64>::new();
        let mut to_fill = bs.iter().collect::<HashSet<_>>();
        let mut to_explore = VecDeque::<u64>::new();
        ops.insert(1, 0);
        to_explore.push_back(1);
        to_fill.remove(&1);
        while !to_fill.is_empty() {
            let node = to_explore.pop_front().unwrap();
            let next_nodes = next_nodes(node);
            for next_node in next_nodes {
                if ops.get(&next_node).is_none() {
                    ops.insert(next_node, ops[&node] + 1);
                    to_explore.push_back(next_node);
                    to_fill.remove(&next_node);
                }
            }
        }

        // Knapsack
        let ops = bs.iter().map(|b| ops[&b]).collect::<Vec<_>>();
        println!("{}", knapsack_01(&cs.into_iter().zip(ops.into_iter()).collect::<Vec<_>>(), k as u64));
    }
}

mod scanner {
    use std::collections::{HashSet, VecDeque};
    use std::{any::type_name, io::stdin, str::FromStr};

    pub struct Scanner {
        tokens: VecDeque<String>,
        delimiters: Option<HashSet<char>>,
    }
    impl Scanner {
        pub fn new() -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: None,
            }
        }

        pub fn with_delimiters(delimiters: &[char]) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: Some(delimiters.iter().copied().collect()),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            let token = loop {
                let front = self.tokens.pop_front();
                if let Some(token) = front {
                    break token;
                }
                self.receive_input();
            };
            token
                .parse::<T>()
                .unwrap_or_else(|_| panic!("input {} isn't a {}", token, type_name::<T>()))
        }

        pub fn next_line(&mut self) -> String {
            assert!(self.tokens.is_empty(), "You have unprocessed token");
            let mut line = String::new();
            stdin().read_line(&mut line).expect("Failed to read.");
            line.pop();
            line
        }

        fn receive_input(&mut self) {
            let mut line = String::new();
            stdin().read_line(&mut line).expect("Failed to read.");
            if let Some(delimiters) = &self.delimiters {
                for token in line.split(|c| delimiters.contains(&c)) {
                    self.tokens.push_back(token.to_string());
                }
            } else {
                for token in line.split_whitespace() {
                    self.tokens.push_back(token.to_string());
                }
            }
        }
    }
}
#[allow(unused_imports)]
use scanner::*;
