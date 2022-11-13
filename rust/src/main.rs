#![allow(
    unused_imports,
    dead_code,
    clippy::needless_range_loop,
    clippy::comparison_chain
)]
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
};

fn knapsack(
    cycles: &[usize],
    start: usize,
    target: usize,
    mem: &mut HashMap<(usize, usize), Option<usize>>,
) -> Option<usize> {
    let key = (start, target);
    if let Some(&m) = mem.get(&key) {
        return m;
    }
    if target == 0 {
        return Some(0);
    } else if target > 0 && start == cycles.len() {
        return None;
    }

    let mut same_e = 1;
    while start + same_e < cycles.len() && cycles[start + same_e] == cycles[start] {
        same_e += 1;
    }

    let mut min = None;
    for included in 0..=same_e {
        if target >= cycles[start] * included {
            if let Some(m) = knapsack(cycles, start + same_e, target - cycles[start] * included, mem) {
                min = Some(min.unwrap_or(std::usize::MAX).min(m + included));
            }
        } else {
            break;
        }
    }
    mem.insert(key, min);
    min
}

fn main() {
    let mut sc = Scanner::new();
    let test_cases = sc.next::<usize>();
    for case_number in 1..=test_cases {
        let n = sc.next::<usize>();
        let mut p = Vec::with_capacity(n);
        for _ in 0..n {
            p.push(sc.next::<usize>() - 1);
        }
        let g = SuccessorGraph::from_successors(p);
        let mut cycles = Vec::new();
        let mut added = vec![false; n];
        let mut i = 0;
        while i < n {
            let c = g.get_cycle(i);
            cycles.push(c.len());
            for node in c {
                added[node] = true;
            }
            while i < n && added[i] {
                i += 1;
            }
        }
        cycles.sort();
        let mut mem = HashMap::new();
        print!("Case #{}:", case_number);
        for target in 1..=n {
            let search_res = cycles.binary_search(&target);
            if search_res.is_ok() {
                // there's a cycle matches the target
                // no extra swap needed
                print!(" 0");
                continue;
            }
            let search_res = search_res.err().unwrap();
            if search_res < cycles.len() {
                // there's a cycle bigger than target
                // one swap is needed to trim the cycle
                print!(" 1");
                continue;
            }
            let mut it = cycles.iter().enumerate().rev();
            let mut t = target - it.next().unwrap().1;
            // include the biggest cycle first
            let mut count = 0;
            for (i, &cycle) in it {
                if cycle < t {
                    // include the current cycle
                    t -= cycle;
                    count += 1;
                } else {
                    // cycle bigger or eq to target
                    if cycles[0..=i].binary_search(&t).is_ok() {
                        count += 1;
                    } else {
                        count += 2;
                    }
                    break;
                }
            }
            count =
                count.min(knapsack(&cycles, 0, target, &mut mem).unwrap_or(std::usize::MAX) - 1);
            print!(" {}", count);
        }
        println!();
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

const USIZE_BITS: u32 = 64;
/// O(1)
pub fn highest_one_bit(x: usize) -> u32 {
    // assuming 64 bit system
    USIZE_BITS - x.leading_zeros()
}

mod successor_graph {
    use crate::highest_one_bit;

    pub struct SuccessorGraph {
        // kth successors of node x will be logkth_successors[log(k)][x]
        logkth_successors: Vec<Vec<usize>>,
    }
    impl SuccessorGraph {
        pub fn new() -> Self {
            Self {
                logkth_successors: vec![Vec::new()],
            }
        }
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                logkth_successors: vec![Vec::with_capacity(capacity)],
            }
        }
        pub fn from_successors(successors: Vec<usize>) -> Self {
            Self {
                logkth_successors: vec![successors],
            }
        }

        /// O(log(k))
        pub fn add_node(&mut self, successor: usize) -> usize {
            let node = self.logkth_successors[0].len();
            self.logkth_successors[0].push(successor);
            for logk in 1..self.logkth_successors.len() {
                let s = self.logkth_successors[logk - 1][node];
                let s2 = self.logkth_successors[logk - 1][s];
                self.logkth_successors[logk].push(s2);
            }
            node
        }

        /// O(nlog(k))
        pub fn index_upto_kth_successor(&mut self, k: usize) {
            let logkth_successors = &mut self.logkth_successors;
            while logkth_successors.len() < highest_one_bit(k) as usize {
                let n = logkth_successors.len();
                let lognth_successors = &logkth_successors[n - 1];
                let mut lognp1th_successors = Vec::with_capacity(lognth_successors.len());
                for i in 0..lognth_successors.len() {
                    let s = lognth_successors[i];
                    let ss = lognth_successors[s];
                    lognp1th_successors.push(ss);
                }
                logkth_successors.push(lognp1th_successors);
            }
        }

        /// O(log(k))
        pub fn get_kth_successor(&self, node: usize, k: usize) -> usize {
            let logkth_successors = &self.logkth_successors;
            assert!(
                logkth_successors.len() >= highest_one_bit(k) as usize,
                "Need to index upto kthsuccessor first"
            );
            let mut node = node;
            let mut k = k;
            let mut logk = 0;
            while k > 0 {
                if k % 2 == 1 {
                    node = logkth_successors[logk][node];
                }
                k /= 2;
                logk += 1;
            }
            node
        }

        pub fn get_successor(&self, node: usize) -> usize {
            self.logkth_successors[0][node]
        }

        /// O(n)
        pub fn get_cycle(&self, start: usize) -> Vec<usize> {
            let mut a = self.get_successor(start);
            let mut b = self.get_successor(self.get_successor(start));
            while a != b {
                a = self.get_successor(a);
                b = self.get_successor(b);
                b = self.get_successor(b);
            }
            // at this point, b is s away from f.
            a = start;
            while a != b {
                a = self.get_successor(a);
                b = self.get_successor(b);
            }
            let first = a;
            let mut node = self.get_successor(first);
            let mut cycle = vec![first];
            while node != first {
                cycle.push(node);
                node = self.get_successor(node);
            }
            cycle
        }
    }
}
#[allow(unused_imports)]
use successor_graph::*;
