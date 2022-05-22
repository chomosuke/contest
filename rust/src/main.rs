#![allow(unused_imports)]
use std::any::type_name;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::str::FromStr;

struct Scanner {
    tokens: VecDeque<String>,
    delimiters: Vec<char>,
}
impl Scanner {
    #[allow(dead_code)]
    fn new() -> Scanner {
        Scanner {
            tokens: VecDeque::new(),
            delimiters: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn add_delimiter(&mut self, delimiter: char) {
        self.remove_delimiter(delimiter);
        self.delimiters.push(delimiter);
    }

    #[allow(dead_code)]
    fn remove_delimiter(&mut self, delimiter: char) {
        let mut j = 0;
        let mut i = 0;
        while i < self.delimiters.len() {
            while self.delimiters[i] == delimiter {
                i += 1;
            }
            self.delimiters[j] = self.delimiters[i];
            j += 1;
            i += 1;
        }
        self.delimiters.truncate(j);
    }

    #[allow(dead_code)]
    fn next<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: Debug,
    {
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
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read.");
        for tokens in buffer.split_whitespace() {
            for token in tokens.split(&self.delimiters[..]) {
                self.tokens.push_back(String::from(token));
            }
        }
    }

    #[allow(dead_code)]
    fn next_line(&mut self) -> String {
        if !self.tokens.is_empty() {
            panic!("You have unprocessed token");
        }
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read.");
        buffer
    }
}

type Count = usize;

#[derive(Clone)]
struct CountMap<K: Eq + Hash> {
    hash_map: HashMap<K, Count>,
}
impl<K: Eq + Hash> CountMap<K> {
    #[allow(dead_code)]
    fn new() -> CountMap<K> {
        CountMap {
            hash_map: HashMap::<K, Count>::new(),
        }
    }

    #[allow(dead_code)]
    fn get(&self, key: &K) -> &Count {
        return self.hash_map.get(key).unwrap_or(&0);
    }

    #[allow(dead_code)]
    fn change(&mut self, key: K, value: Count) {
        let next = self.get(&key) + value;
        self.hash_map.insert(key, next);
    }
}

fn main() {
    let mut sc = Scanner::new();
    sc.add_delimiter(':');

    let h1 = sc.next::<i128>();
    let m1 = sc.next::<i128>();
    let h2 = sc.next::<i128>();
    let m2 = sc.next::<i128>();

    let m1 = m1 + h1 * 60;
    let m2 = m2 + h2 * 60;

    let m3 = (m1 + m2) / 2;
    let h3 = m3 / 60;
    let m3 = m3 % 60;

    println!("{:02}:{:02}", h3, m3);
}

