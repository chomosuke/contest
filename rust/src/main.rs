use std::io::stdin;
use std::str::FromStr;
use std::fmt::Debug;
use std::any::type_name;
use std::collections::VecDeque;

struct Scanner {
    tokens: VecDeque<String>,
}

impl Scanner {
    pub fn new() -> Scanner {
        return Scanner {
            tokens: VecDeque::new(),
        };
    }

    pub fn next<T: FromStr>(&mut self) -> T where
        <T as FromStr>::Err: Debug {
        let mut token = loop {
            let front = self.tokens.pop_front();
            if let Some(token) = front {
                break token;
            }
            self.recieve_input();
        };
        return token.parse::<T>()
            .expect(&format!("input isn't a {}", type_name::<T>()));
    }

    fn recieve_input(&mut self) {
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Failed to read.");
        for token in  buffer.split_whitespace() {
            self.tokens.push_back(String::from(token));
        }
    }

    pub fn next_line(&mut self) -> String {
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
    
}
