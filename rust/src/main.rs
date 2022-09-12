#![allow(unused_imports)]
use std::any::type_name;
use std::cmp::*;
use std::collections::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::ops::Bound::*;
use std::str::FromStr;

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

trait BinarySearchable<T> {
    fn binary_search_leq(&self, x: &T) -> usize;
    fn binary_search_geq(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearchable<T> for [T] {
    fn binary_search_leq(&self, x: &T) -> usize {
        self.binary_search_by(|p| {
            let r = p.cmp(x);
            if r == Ordering::Greater {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .err()
        .unwrap()
    }

    fn binary_search_geq(&self, x: &T) -> usize {
        self.binary_search_by(|p| {
            let r = p.cmp(x);
            if r == Ordering::Less {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .err()
        .unwrap()
    }
}

#[allow(dead_code)]
type I = i128;
#[allow(dead_code)]
type U = usize;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next::<U>();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(sc.next::<I>());
    }
    let sum: I = a.iter().sum();
    let target = sum / 3;
    if sum != target * 3 {
        println!("0");
        return;
    }
    let mut sum = 0;
    let mut target_indexes = Vec::new();
    for (i, ai) in a.iter().enumerate().take(a.len() - 1) {
        sum += ai;
        if sum == target {
            target_indexes.push(i);
        }
    }
    sum = 0;
    let mut target_indexes_rev = Vec::new();
    for (i, ai) in a.iter().enumerate().rev().take(a.len() - 1) {
        sum += ai;
        if sum == target {
            target_indexes_rev.push(i - 1);
        }
    }
    let target_indexes_rev: Vec<U> = target_indexes_rev.into_iter().rev().collect();
    let (mut i, mut j) = (0, 0);
    let mut count = 0;
    'outer: while i < target_indexes.len() && j < target_indexes_rev.len() {
        while target_indexes[i] >= target_indexes_rev[j] {
            j += 1;
            if j >= target_indexes_rev.len() {
                break 'outer;
            }
        }
        count += target_indexes_rev.len() - j;
        i += 1;
    }
    println!("{}", count);
}
