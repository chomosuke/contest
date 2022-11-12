#![allow(
    unused_imports,
    dead_code,
    clippy::needless_range_loop,
    clippy::comparison_chain
)]
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
};

fn get_min(b: usize, l: usize, n: usize, mem: &mut HashMap<(usize, usize ,usize), usize>) -> usize {
    let key = (b, l, n);
    if let Some(&m) = mem.get(&key) {
        return m;
    }
    if l == 0 {
        return 0;
    }
    let mut min = get_min(b + 1, l - 1, n, mem) + 1;
    if b != n {
        min = min.min(get_min(b, l, b, mem) + 4);
    }
    if n != 0 && n <= l{
        min = min.min(get_min(b + n, l - n, n, mem) + 2);
    }
    mem.insert(key, min);
    min
}

fn main() {
    let mut sc = Scanner::new();
    let test_cases = sc.next::<usize>();
    let mut mem = HashMap::new();
    for case_number in 1..=test_cases {
        let l = sc.next::<usize>();
        println!("Case #{}: {}", case_number, get_min(0, l, 0, &mut mem));
    }
}

mod scanner {
    use std::collections::{HashSet, VecDeque};
    use std::{any::type_name, io::stdin, str::FromStr};

    pub struct Scanner {
        tokens: VecDeque<String>,
        delimiters: Option<HashSet<char>>,
    }
    impl Scanner {
        pub fn new() -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: None,
            }
        }

        pub fn with_delimiters(delimiters: &[char]) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: Some(delimiters.iter().copied().collect()),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
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

        pub fn next_line(&mut self) -> String {
            assert!(self.tokens.is_empty(), "You have unprocessed token");
            let mut line = String::new();
            stdin().read_line(&mut line).expect("Failed to read.");
            line.pop();
            line
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
    }
}
#[allow(unused_imports)]
use scanner::*;
