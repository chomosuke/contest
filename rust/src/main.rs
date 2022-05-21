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
    let t = sc.next::<u16>();

    for _ in 0..t {
        let n = sc.next::<usize>();
        let m = sc.next::<usize>();

        let mut board = Vec::<Vec<u64>>::with_capacity(n);
        for _ in 0..n {
            let mut row = Vec::<u64>::with_capacity(m);
            for _ in 0..m {
                row.push(sc.next::<u64>());
            }
            board.push(row);
        }

        // now find max sum
        let mut max_sum = 0;
        for i in 0..n {
            for j in 0..m {
                max_sum = max(max_sum, sum(&board, i, j));
            }
        }

        println!("{}", max_sum);
    }
}

fn sum(board: &Vec<Vec<u64>>, i: usize, j: usize) -> u64 {
    let mut sum: u64 = 0;

    for r in 0..board.len() {
        if r == i {
            sum += board[i][j];
        } else {
            let c = r as i64 - i as i64 + j as i64;
            if c >= 0 && (c as usize) < board[r].len() {
                sum += board[r][c as usize];
            }
            let c = i as i64 - r as i64  + j as i64;
            if c >= 0 && (c as usize) < board[r].len() {
                sum += board[r][c as usize];
            }
        }
    }

    sum
}

