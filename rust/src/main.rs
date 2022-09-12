#![allow(unused_imports)]
use std::any::type_name;
use std::cmp::{max, min};
use std::collections::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::str::FromStr;
use std::ops::Bound::*;

struct Scanner {
    tokens: VecDeque<String>,
    delimiters: Option<HashSet<char>>,
}
impl Scanner {
    #[allow(dead_code)]
    fn new() -> Scanner {
        Scanner {
            tokens: VecDeque::new(),
            delimiters: None,
        }
    }

    #[allow(dead_code)]
    fn with_delimiters(delimiters: &[char]) -> Scanner {
        Scanner {
            tokens: VecDeque::new(),
            delimiters: Some(delimiters.iter().copied().collect()),
        }
    }

    #[allow(dead_code)]
    fn next<T: FromStr>(&mut self) -> T {
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

    #[allow(dead_code)]
    fn next_line(&mut self) -> String {
        if !self.tokens.is_empty() {
            panic!("You have unprocessed token");
        }
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read.");
        line
    }
}

type Count = usize;

#[derive(Clone)]
struct MultiSet<E: Eq + Hash> {
    hash_map: HashMap<E, Count>,
}
impl<E: Eq + Hash> MultiSet<E> {
    #[allow(dead_code)]
    fn new() -> MultiSet<E> {
        MultiSet {
            hash_map: HashMap::<E, Count>::new(),
        }
    }

    #[allow(dead_code)]
    fn count(&self, e: &E) -> &Count {
        return self.hash_map.get(e).unwrap_or(&0);
    }

    #[allow(dead_code)]
    fn insert(&mut self, e: E) {
        let next = self.count(&e) + 1;
        self.hash_map.insert(e, next);
    }
}

#[allow(dead_code)]
type I = i128;
#[allow(dead_code)]
type U = usize;

fn main() {
    let mut sc = Scanner::new();
    let str1 = sc.next_line();
    let str2 = sc.next_line();
    println!("{}", distance(str1.as_bytes(), str2.as_bytes(), 0, 0, &mut HashMap::new()))
}

fn distance(str1: &[u8], str2: &[u8], i1: U, i2: U, memoize: &mut HashMap<(U, U), U>) -> U {
    if let Some(&d) = memoize.get(&(i1, i2)) {
        return d;
    }
    if i1 == str1.len() {
        return str2.len() - i2;
    }
    if i2 == str2.len() {
        return str1.len() - i1;
    }
    let cost = if str1[i1] == str2[i2] { 0 } else { 1 };
    let d = (distance(str1, str2, i1 + 1, i2, memoize) + 1)
                   .min(distance(str1, str2, i1, i2 + 1, memoize) + 1)
                   .min(distance(str1, str2, i1 + 1, i2 + 1, memoize) + cost);
    memoize.insert((i1, i2), d);
    d
}
