use std::io::stdin;
use std::str::FromStr;
use std::fmt::Debug;
use std::any::type_name;
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

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

fn main() {
    let mut sc = Scanner::new();
    let t = sc.next::<i128>();
    for _ in 0..t {
        let n = sc.next::<usize>();
        let mut counts = HashMap::<i128, i128>::new();
        for _ in 0..n {
            let k = sc.next::<i128>();
            counts.insert(k, *counts.get(&k).unwrap_or(&(0 as i128)) + 1);
        } // k is the filling flavor, v is the count
        let mut counts_of_counts = HashMap::<i128, i128>::new();
        for count_of_counts in counts.values() {
            counts_of_counts.insert(
                *count_of_counts,
                *counts_of_counts.get(count_of_counts).unwrap_or(&(0 as i128)) + 1
            );
        } // k is the count, and v is the number of flavor with that count.
        let counts_of_counts = Vec::from_iter(counts_of_counts.iter());
        // in another word, v is the length of the string, and k is number of those string.

        let mut max_k = counts_of_counts[0].0;
        let mut biggest_k_i: usize = 0;
        for i in 1..counts_of_counts.len() {
            if counts_of_counts[i].0 > &max_k {
                max_k = counts_of_counts[i].0;
                biggest_k_i = i;
            }
        }

        let biggest_k = counts_of_counts[biggest_k_i];

        let mut sum: i128 = - biggest_k.0 * biggest_k.1;
        for (k, v) in counts_of_counts {
            sum += k * v;
        }
        println!("{}", sum / (biggest_k.0 - 1) + biggest_k.1 - 1);
    }
}
