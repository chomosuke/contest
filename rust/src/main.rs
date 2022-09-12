#![allow(unused_imports)]
use std::any::type_name;
use std::cmp::{max, min};
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

#[allow(dead_code)]
type I = i128;
#[allow(dead_code)]
type U = usize;

fn main() {
    let mut sc = Scanner::new();
    let m = sc.next::<I>();
    let n = sc.next::<U>();
    println!("{}", count(&vec![0; n], m))
}

// symbols are n u ( )
fn count(row: &[u8], m: I) -> I {
    if m == 0 {
        for &x in row {
            if x == b'u' {
                return 0;
            }
        }
        return 1;
    }
    enumerate(row, &mut vec![0; row.len()], 0, m)
}

fn enumerate(row: &[u8], next_row: &mut Vec<u8>, i: U, m: I) -> I {
    if i == next_row.len() {
        return count(next_row, m - 1);
    }
    if row[i] == b'u' {
        next_row[i] = b'n';
        return enumerate(row, next_row, i + 1, m);
    }
    if i > 0 && next_row[i - 1] == b'(' {
        next_row[i] = b')';
        return enumerate(row, next_row, i + 1, m);
    }
    let mut sum = 0;
    if i < next_row.len() - 1 {
        next_row[i] = b'(';
        sum += enumerate(row, next_row, i + 1, m);
    }
    next_row[i] = b'u';
    sum += enumerate(row, next_row, i + 1, m);
    sum
}
