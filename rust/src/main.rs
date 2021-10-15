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
    let mut first_score = Vec::<i32>::new();
    let mut second_score = Vec::<i32>::new();
    let mut sum: i64 = 0;
    let mut first_last = true;
    for _ in 0..n {
        let a = sc.next::<i32>();
        if a > 0 {
            first_score.push(a);
            first_last = true;
        } else {
            second_score.push(a);
            first_last = false;
        }
        sum += a as i64;
    }
    if sum > 0 {
        println!("first");
    } else if sum < 0 {
        println!("second");
    } else {
        for (f, s) in first_score.iter().zip(second_score.iter()) {
            if f + s > 0 {
                println!("first");
                return;
            } else if f + s < 0 {
                println!("second");
                return;
            }
        }
        if first_score.len() > second_score.len() {
            println!("first");
        } else if first_score.len() < second_score.len() {
            println!("second");
        } else if first_last {
            println!("first");
        } else {
            println!("second");
        }
    }
}
