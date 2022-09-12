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
fn hash_map_to_vec<K: Clone, V: Clone>(hash_map: &HashMap<K, V>) -> Vec<(K, V)> {
    return hash_map
        .iter()
        .map(|(k, v)| ((*k).clone(), (*v).clone()))
        .collect::<Vec<(K, V)>>();
}

#[allow(dead_code)]
type I = i128;
#[allow(dead_code)]
type U = usize;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next::<U>();
    // key is the end element and value is the size of candidate list
    let mut candidates = BTreeMap::<I, I>::new();
    for _ in 0..n {
        let e = sc.next::<I>();
        let new_length = if let Some((_, length)) = candidates.range(..e).max() {
            // end element will be smaller than e
            // can append e to the list
            length + 1
        } else {
            1
        };
        candidates.insert(e, new_length);
        // and delete list with the same length and bigger end element
        let mut to_be_deleted = Vec::new();
        let list_with_bigger_end = candidates.range((Excluded(e), Unbounded));
        for (end, length) in list_with_bigger_end {
            if end > &e && length == &new_length {
                to_be_deleted.push(*end);
            } else {
                break;
            }
        }
        for k in to_be_deleted {
            candidates.remove(&k);
        }
    }
    println!("{}", candidates.range(..).max().unwrap().1);
}
