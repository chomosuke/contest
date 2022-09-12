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

#[allow(dead_code)]
type I = i128;
#[allow(dead_code)]
type U = usize;

fn main() {
    let mut sc = Scanner::new();
    let t = sc.next::<U>();
    for _ in 0..t {
        let n = sc.next::<U>();
        let q = sc.next::<U>();
        let mut a = Vec::with_capacity(n);
        let mut s = HashMap::<I, Vec<U>>::with_capacity(q);
        for _ in 0..n {
            a.push(sc.next::<I>());
        }
        a.sort();
        let mut sum_so_far = Vec::with_capacity(n);
        sum_so_far.push(0);
        for ai in &a {
            sum_so_far.push(sum_so_far.last().unwrap() + ai);
        }
        for i in 0..q {
            let si = sc.next::<I>();
            let mut ls = s.remove(&si).unwrap_or_default();
            ls.push(i);
            s.insert(si, ls);
        }
        let mut ans = vec![false; q];
        tests(&a, &sum_so_far, &s, &mut ans);
        for p in ans {
            if p {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

fn tests(a: &[I], sum_so_far: &[I], s: &HashMap<I, Vec<U>>, ans: &mut [bool]) {
    if let Some(ls) = s.get(&(sum_so_far.last().unwrap() - sum_so_far[0])) {
        for &l in ls {
            ans[l] = true;
        }
    }
    if a.len() <= 1 {
        return;
    }
    let mid = (a[0] + a.last().unwrap()) / 2;
    let split = a
        .binary_search_by(|ai| {
            let r = ai.cmp(&mid);
            if r == Ordering::Greater {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .err()
        .unwrap();
    if split == a.len() {
        return;
    }
    tests(&a[..split], &sum_so_far[..(split + 1)], s, ans);
    tests(&a[split..], &sum_so_far[split..], s, ans);
}
