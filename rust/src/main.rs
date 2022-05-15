#![allow(unused_imports)]
use std::any::type_name;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::str::FromStr;

struct Scanner {
    tokens: VecDeque<String>,
}
impl Scanner {
    #[allow(dead_code)]
    fn new() -> Scanner {
        Scanner {
            tokens: VecDeque::new(),
        }
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
            .unwrap_or_else(|_| panic!("input isn't a {}", type_name::<T>()))
    }

    fn receive_input(&mut self) {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read.");
        for token in buffer.split_whitespace() {
            self.tokens.push_back(String::from(token));
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
    let n = sc.next::<i128>();
    let k = sc.next::<i128>();
    let mut al = Vec::with_capacity(n as usize);
    for i in 1..=n {
        al.push((sc.next::<i128>(), i));
    }
    al.sort_unstable();
    let mut m = 0;
    let mut ts = 0;
    let mut il = Vec::new();
    for inst in al {
        ts += inst.0;
        if ts <= k {
            m += 1;
            il.push(inst.1);
        } else {
            break;
        }
    }
    println!("{}", m);
    for i in il {
        print!("{} ", i);
    }
}
