#![allow(unused_imports, dead_code)]
use std::{
    cmp::{min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
};

trait MostPopular {
    fn increase_popularity(&mut self, content_id: u64);
    fn most_popular(&self) -> u64;
    fn decrease_popularity(&mut self, content_id: u64);
}

struct PopularitySorter {
    // (popularity, content_id)
    map: BTreeSet<(u64, u64)>,
    // key: id, value: popularity
    pop_map: HashMap<u64, u64>,
}

impl MostPopular for PopularitySorter {
    fn increase_popularity(&mut self, content_id: u64) {
        let mut popularity = self.pop_map.get(&content_id);
        if popularity.is_none() {
            self.pop_map.insert(content_id, 0);
            popularity = Some(&0);
        } else {
            self.map.remove(&(*popularity.unwrap(), content_id));
        }
        let popularity = *popularity.unwrap() + 1;
        self.pop_map.insert(content_id, popularity);
        self.map.insert((popularity, content_id));
    }

    fn most_popular(&self) -> u64 {
        self.map.last().map(|e| e.1).unwrap_or(0)
    }

    fn decrease_popularity(&mut self, content_id: u64) {
        let mut popularity = self.pop_map.get(&content_id);
        if popularity.is_none() {
            self.pop_map.insert(content_id, 0);
            popularity = Some(&0);
        } else {
            self.map.remove(&(*popularity.unwrap(), content_id));
        }
        let popularity = *popularity.unwrap();
        if popularity <= 1 {
            self.pop_map.insert(content_id, popularity - 1);
            self.map.insert((popularity - 1, content_id));
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut popularity_tracker = PopularitySorter {
            map: BTreeSet::new(),
            pop_map: HashMap::new(),
        };
        popularity_tracker.increase_popularity(7);
        popularity_tracker.increase_popularity(7);
        popularity_tracker.increase_popularity(8);
        assert_eq!(popularity_tracker.most_popular(), 7); // returns 7
        popularity_tracker.increase_popularity(8);
        popularity_tracker.increase_popularity(8);
        assert_eq!(popularity_tracker.most_popular(), 8); // returns 8
        popularity_tracker.decrease_popularity(8);
        popularity_tracker.decrease_popularity(8);
        assert_eq!(popularity_tracker.most_popular(), 7); // returns 7
        popularity_tracker.decrease_popularity(7);
        popularity_tracker.decrease_popularity(7);
        popularity_tracker.decrease_popularity(8);
        assert_eq!(popularity_tracker.most_popular(), 0); // returns -1 since there is no content with popularity greater than 0
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
