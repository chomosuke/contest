#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{BufReader, stdin},
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let n = sc.next::<usize>();
    let mut costs = Vec::with_capacity(n);
    for _ in 0..n {
        costs.push(sc.next::<u64>());
    }
    let m = sc.next::<usize>();
    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        edges.push((sc.next::<usize>() - 1, sc.next::<usize>() - 1));
    }
    let mut adj_nodes = vec![Vec::new(); n];
    let mut rev_adj_n = vec![Vec::new(); n];
    for (u, v) in edges {
        adj_nodes[u].push(v);
        rev_adj_n[v].push(u);
    }

    // To find the strongly connected components, we want to always start at the
    // end of the DAG produced by collapsing all components.
    //
    // If we do a dfs and then start from the leafs, we'll find all strongly connect components
    let mut visited = vec![false; n];
    let mut finish_order = Vec::new();
    fn dfs(node: usize, adj_nodes: &[Vec<usize>], visited: &mut [bool], finish_order: &mut Vec<usize>) {
        assert!(!visited[node]);
        visited[node] = true;
        for &adj_node in &adj_nodes[node] {
            if !visited[adj_node] {
                dfs(adj_node, adj_nodes, visited, finish_order);
            }
        }
        finish_order.push(node);
    }
    for i in 0..n {
        if !visited[i] {
            dfs(i, &adj_nodes, &mut visited, &mut finish_order);
        }
    }

    let mut visited = vec![false; n];
    let mut sccs = Vec::new();
    for node in finish_order.into_iter().rev() {
        if !visited[node] {
            let mut scc = Vec::new();
            dfs(node, &rev_adj_n, &mut visited, &mut scc);
            sccs.push(scc);
        }
    }

    let costs = sccs.into_iter().map(|scc| {
        let costs = scc.into_iter().map(|n| costs[n]).collect::<Vec<_>>();
        let min = costs.iter().min().unwrap();
        (*min, costs.iter().filter(|&c| c == min).count())
    }).collect::<Vec<_>>();

    let cost: u64 = costs.iter().map(|&(c, _)| c).sum();
    let mut ways = 1;
    for (_, w) in costs {
        ways *= w;
        ways %= 1000000007;
    }
    println!("{cost} {ways}");
}

mod scanner {
    use std::collections::{HashSet, VecDeque};
    use std::io::{BufReader, Read, Lines};
    use std::{any::type_name, io::BufRead, str::FromStr};

    pub struct Scanner<R: Read> {
        tokens: VecDeque<String>,
        delimiters: Option<HashSet<char>>,
        lines: Lines<BufReader<R>>,
    }
    impl<R: Read> Scanner<R> {
        pub fn new(source: R) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: None,
                lines: BufReader::new(source).lines(),
            }
        }

        pub fn with_delimiters(source: R, delimiters: &[char]) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: Some(delimiters.iter().copied().collect()),
                lines: BufReader::new(source).lines(),
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
            let line = self.lines.next().and_then(|e| e.ok()).expect("Failed to read.");
            line
        }

        fn receive_input(&mut self) {
            let line = self.lines.next().and_then(|e| e.ok()).expect("Failed to read.");
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
