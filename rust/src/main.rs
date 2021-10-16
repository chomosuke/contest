use std::io::stdin;
use std::str::FromStr;
use std::fmt::Debug;
use std::any::type_name;
use std::collections::{HashSet, VecDeque};

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
    let n = sc.next::<usize>();
    let mut titles = Vec::<Vec<u8>>::new();
    for _ in 0..n {
        titles.push(sc.next_line().into_bytes());
    }
    let mut title_len = 1;
    loop {
        let mut sub_strs =  HashSet::<&[u8]>::new();
        for title in titles.iter() {
            for i in 0..(title.len() - title_len) {
                sub_strs.insert(&title[i..(i + title_len)]);
            }
        }
        // check everything
        let mut guess = vec!['a' as u8; title_len];
        'trail_with_current_len: loop {
            if !sub_strs.contains(&*guess) {
                println!("{}", String::from_utf8_lossy(&*guess));
                return;
            }
            let mut last = guess.len() - 1;
            guess[last] += 1;
            while guess[last] > 'z' as u8 {
                if last == 0 {
                    break 'trail_with_current_len;
                }
                guess[last] = 'a' as u8;
                last -= 1;
                guess[last] += 1;
            }
        }
        title_len += 1;
    }
}
