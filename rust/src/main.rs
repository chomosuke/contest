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
    let t = sc.next::<usize>();
    for _ in 0..t {

        // input
        let n = sc.next::<usize>();
        let mut a = Vec::<usize>::with_capacity(n);
        for _ in 0..n {
            a.push(sc.next::<usize>() - 1);
        }

        // minimize max_c;
        let mut existing_c = HashMap::<usize, usize>::new();
        let mut count = 0;
        for i in 0..n {
            if !existing_c.contains_key(&a[i]) {
                existing_c.insert(a[i], count);
                count += 1;
            }
        }
        let max_c = count - 1;
        for i in 0..n {
            let temp = existing_c.get(&a[i]).unwrap();
            a[i] = *temp;
        }

        let mut num_before = Vec::<Vec<usize>>::with_capacity(n + 1);
        // num_before[i][c] = number of c before i exclusive.

        num_before.push(vec![0; max_c + 1]);
        for i in 1..=a.len() {
            num_before.push(Vec::<usize>::with_capacity(max_c + 1));
            for c in 0..=max_c {
                let prev = num_before[i - 1][c];
                num_before[i].push(prev);
            }
            num_before[i][a[i - 1]] += 1;
        }

        let mut max_num_c_between = HashMap::<(usize, usize), usize>::new();
        // max_num_c_between.get((start, end)); the maximum number of same c that is in [start, end)

        let mut max_p = 0;
        for c in 0..=max_c {
            let mut start = 0 as usize;
            let mut end = n;
            'outer: while end > start {
                // now count the max_num_c_between
                if !max_num_c_between.contains_key(&(start, end)) {
                    max_num_c_between.insert((start, end), 0);
                    for c in 0..=max_c {
                        max_num_c_between.insert(
                            (start, end),
                            max(
                                num_before[end][c] - num_before[start][c],
                                *max_num_c_between.get(&(start, end)).unwrap()
                            ),
                        );
                    }
                }
                max_p = max(
                    max_p,
                    num_before[start][c] * 2 + *max_num_c_between.get(&(start, end)).unwrap()
                );
                // now move start and end
                let prev_before = num_before[start][c];
                while num_before[start][c] == prev_before {
                    start += 1;
                    if start >= n {
                        break 'outer;
                    }
                }
                while num_before[n][c] - num_before[end][c] == prev_before {
                    end -= 1;
                }
            }
        }

        println!("{}", max_p);
    }
}
