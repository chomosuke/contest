#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{BufReader, stdin},
};

fn get_max_good(
    start: u64,
    i: usize,
    arr: &[u64],
    mem: &mut HashMap<(u64, usize), u64>,
    l: u64,
    r: u64,
    h: u64,
) -> u64 {
    if i >= arr.len() {
        return 0;
    }
    let k = (start, i);
    if !mem.contains_key(&k) {
        let start1 = (start + arr[i]) % h;
        let good1 = if l <= start1 && start1 <= r { 1 } else { 0 };
        let start2 = (start + arr[i] - 1) % h;
        let good2 = if l <= start2 && start2 <= r { 1 } else { 0 };
        let max_good = max(
            get_max_good(start1, i + 1, arr, mem, l, r, h) + good1,
            get_max_good(start2, i + 1, arr, mem, l, r, h) + good2,
        );
        mem.insert(k, max_good);
    }
    *mem.get(&k).unwrap()
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let n = sc.next::<usize>();
    let h = sc.next::<u64>();
    let l = sc.next::<u64>();
    let r = sc.next::<u64>();
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(sc.next::<u64>());
    }
    println!("{}", get_max_good(0, 0, &arr, &mut HashMap::new(), l, r, h))
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
