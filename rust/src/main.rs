#![allow(unused_imports)]
use std::io::stdin;
use std::str::FromStr;
use std::fmt::Debug;
use std::any::type_name;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct Scanner {
    tokens: VecDeque<String>,
}
impl Scanner {
    #[allow(dead_code)]
    fn new() -> Scanner {
        return Scanner {
            tokens: VecDeque::new(),
        };
    }

    #[allow(dead_code)]
    fn next<T: FromStr>(&mut self) -> T where
        <T as FromStr>::Err: Debug {
        let token = loop {
            let front = self.tokens.pop_front();
            if let Some(token) = front {
                break token;
            }
            self.receive_input();
        };
        return token.parse::<T>()
            .expect(&format!("input isn't a {}", type_name::<T>()));
    }

    fn receive_input(&mut self) {
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Failed to read.");
        for token in  buffer.split_whitespace() {
            self.tokens.push_back(String::from(token));
        }
    }

    #[allow(dead_code)]
    fn next_line(&mut self) -> String {
        if !self.tokens.is_empty() {
            panic!("You have unprocessed token");
        }
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Failed to read.");
        return buffer;
    }
}

type Count = usize;

#[derive(Clone)]
struct CountMap<K: Eq + Hash> {
    hash_map: HashMap<K, Count>
}
impl<K: Eq + Hash> CountMap<K> {
    #[allow(dead_code)]
    fn new() -> CountMap<K> {
        return CountMap {
            hash_map: HashMap::<K, Count>::new()
        };
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
    let n = sc.next::<i64>();
    let mut k = sc.next::<usize>();
    let mut facts = Vec::<i64>::new();
    for i in 1..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            k -= 1;
            facts.push(i);
        }
        if k == 0 {
            break;
        }
    }
    if k == 0 {
        println!("{}", facts.pop().unwrap());
    } else {
        let len = facts.len();
        if n == facts[len - 1] * facts[len - 1] {
            k += 1;
        }
        if len < k {
            println!("{}", -1);
        } else {
            println!("{}", n / facts[len - k]);
        }
    }
    
}
