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

#[allow(dead_code)]
type I = i128;
#[allow(dead_code)]
type U = usize;

fn main() {
    let mut sc = Scanner::new();
    let t = sc.next();

    for _ in 0..t {
        sc.next::<I>();
        let s = sc
            .next_line()
            .into_bytes();

        let mut count0 = 0;
        for b in s.iter() {
            if *b == b'0' {
                count0 += 1;
            }
        }

        let mut o = Vec::new();
        for (i, b) in s.iter().enumerate() {
            if *b == b'1' && i < count0 {
                o.push(i + 1);
            }
            if *b == b'0' && i >= count0 {
                o.push(i + 1);
            }
        }

        if o.is_empty() {
            println!("0");
            continue;
        }
        println!("1");
        print!("{}", o.len());
        for b in o.iter() {
            print!(" {}", b);
        }
        println!();
    }
}

