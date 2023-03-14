#![allow(unused_imports, dead_code)]
use std::{
    cmp::{Ordering, min},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
};

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next();
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(sc.next::<usize>() - 1);
    }
    println!("{}", solve(&arr));
}

fn solve(arr: &[usize]) -> f64 {
    // For each unique element in the array.
    // We find the chance of a randomly selected segment contains one of that specific element.
    // Adding all the chance together and we have the expected number of unique elements in a
    // randomly selected segment.

    let mut count = HashMap::<usize, usize>::with_capacity(arr.len());
    for a in arr {
        count.insert(min(*a, arr.len() - 1), *count.get(a).unwrap_or(&0) + 1);
    }

    let mut chance_since_0 = Vec::with_capacity(arr.len() + 1);
    chance_since_0.push(0);
    for i in 1..=arr.len() {
        let last = chance_since_0[i - 1];
        chance_since_0.push(last + count.get(&(i - 1)).unwrap_or(&0));
    }

    let mut pre_indexes = HashMap::<usize, i32>::with_capacity(arr.len());
    let mut sum_uniq = 0;
    for (i, a) in arr.iter().enumerate() {
        // read and update pre_index
        let pre_index = pre_indexes.get(a).unwrap_or(&-1).clone();
        pre_indexes.insert(*a, i as i32);
        sum_uniq += (chance_since_0[i + 1] - chance_since_0[(pre_index + 1) as usize])
            * (chance_since_0[arr.len()] - chance_since_0[i])
            * 2;
    }
    return sum_uniq as f64 / (arr.len() as f64).powi(2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(&[0, 1]), 1.5);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(&[1, 1]), 1.0);
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
