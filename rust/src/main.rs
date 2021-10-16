use std::io::stdin;
use std::str::FromStr;
use std::fmt::Debug;
use std::any::type_name;
use std::collections::VecDeque;

struct Scanner {
    tokens: VecDeque<String>,
}

impl Scanner {
    #[allow(dead_code)]
    pub fn new() -> Scanner {
        return Scanner {
            tokens: VecDeque::new(),
        };
    }

    #[allow(dead_code)]
    pub fn next<T: FromStr>(&mut self) -> T where
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
    let mut sc = Scanner::new();
    let n = sc.next::<i32>();
    let k = sc.next::<i32>();
    let mut disass = 0;
    let mut ass = n;
    for _ in 0..k {
        let m = sc.next::<i32>();
        for i in 0..m {
            if sc.next::<i32>() == i + 1 {
                ass -= 1;
            } else if i != 0 {
                disass += 1;
            }
        }
    }
    println!("{}", disass + ass);
}
