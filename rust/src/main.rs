use std::io::stdin;
use std::str::FromStr;
use std::fmt::Debug;
use std::any::type_name;
use std::collections::{VecDeque};

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
    let n = sc.next::<i64>();
    let m = sc.next::<i64>();
    let asc = sc.next::<i32>() == 0;
    for _ in 0..n {
        sc.next_line();
    }

    // simply implement selection sort
    let mut swaps = Vec::<(i64, i64)>::new();
    for end in (1..=m).rev() {
        for i in 1..end {
            if asc {
                swaps.push((i, i + 1));
            } else {
                swaps.push((i + 1, i));
            }
        }
    }

    println!("{}", m*(m-1)/2);
    for swap in swaps {
        println!("{} {}", swap.0, swap.1);
    }
}
