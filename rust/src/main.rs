#![allow(unused_imports, dead_code)]
use std::{
    cmp::{min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
};

fn main() {
    println!("{}", ps(100_000_000));
}

/// # Examples
///
/// ```
/// println!("Hello world!");
/// let x = 10;
/// assert!(x == 10);
/// fn fun(n: usize) -> usize {
///     if n > 1 {
///         fun(n - 1);
///     } else {
///         return n;
///     }
/// }
/// ```
fn ps(n: usize) -> usize {
    let mut end_fac = if n < 50_000_000 { 20 } else { 30 };
    loop {
        let end = n * end_fac;
        let mut is_prime = vec![true; end + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..end {
            if is_prime[i] {
                let mut j = 2 * i;
                while j <= end {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }

        let r = is_prime
            .into_iter()
            .zip(0..=end)
            .filter(|&(i, _)| i)
            .map(|(_, p)| p)
            .nth(n);
        if let Some(r) = r {
            return r;
        }
        end_fac *= 2;
    }
}

// let mut primes = Vec::<usize>::with_capacity(n);
// primes.push(2);
// let mut c = 3;
// while primes.len() <= n {
//     let mut is_prime = true;
//     for p in primes.iter().take_while(|&&p| p * p <= c) {
//         if c % p == 0 {
//             is_prime = false;
//             break;
//         }
//     }
//     if is_prime {
//         primes.push(c);
//     }
//     c += 1;
// }

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
