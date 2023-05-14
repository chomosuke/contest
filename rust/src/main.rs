#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{BufReader, stdin},
};

fn valid_x(x: u32, counts: &mut BTreeMap<u32, usize>) -> Option<Vec<(u32, u32)>> {
    let max = if let Some(mut max) = counts.last_entry() {
        if *max.get() > 1 {
            *max.get_mut() -= 1;
            *max.key()
        } else {
            max.remove_entry().0
        }
    } else {
        return Some(Vec::new());
    };

    if max > x || !counts.contains_key(&(x - max)) {
        *counts.entry(max).or_default() += 1;
        return None;
    }

    let b = x - max;
    let b_count = *counts.get(&(x - max)).unwrap();
    if b_count > 1 {
        counts.insert(b, b_count - 1);
    } else {
        counts.remove(&b);
    }

    if let Some(mut pairs) = valid_x(max, counts) {
        pairs.push((max, b));
        return Some(pairs);
    } else {
        *counts.entry(max).or_default() += 1;
        *counts.entry(b).or_default() += 1;
        return None;
    }
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let test_cases = sc.next::<usize>();
    'outer: for _ in 0..test_cases {
        let n = sc.next::<usize>();
        let mut counts = BTreeMap::<u32, usize>::new();
        for _ in 0..(2*n) {
            *counts.entry(sc.next::<u32>()).or_default() += 1;
        }
        let mut arr = counts.iter().map(|(&k, _)| k).collect::<Vec<_>>();
        let max = arr.pop().unwrap();
        if *counts.get(&max).unwrap() > 1 {
            arr.push(max);
        }
        for a in arr {
            if let Some(pairs) = valid_x(a + max, &mut counts) {
                println!("YES");
                println!("{}", a + max);
                for (x, y) in pairs.into_iter().rev() {
                    println!("{} {}", x, y);
                }
                continue 'outer;
            }
        }
        println!("NO");
    }
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
