#![allow(
    unused_imports,
    dead_code,
    clippy::needless_range_loop,
    clippy::comparison_chain
)]
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
};

fn to_arr(x: usize) -> Vec<usize> {
    x.to_string()
        .into_bytes()
        .into_iter()
        .map(|d| (d - b'0') as usize)
        .collect()
}

fn to_num(x: &[usize]) -> usize {
    let mut r = 0;
    for (pow, &d) in x.iter().enumerate() {
        r += d.pow(pow as u32);
    }
    r
}

fn get_sum(x: usize) -> u8 {
    x.to_string()
        .into_bytes()
        .into_iter()
        .map(|d| d - b'0')
        .sum()
}

fn count(fixed_digit: &[usize], free_digit: usize, count_dist_mem: &mut CountDistMem) -> usize {
    let fixed_sum = fixed_digit.iter().sum::<usize>();
    let fixed_prod = fixed_digit.iter().product::<usize>();
    let mut count = 0;
    for zero_count in 1..=free_digit {
        let zero_pos_choices =
            binomial(free_digit as i128, zero_count as i128, std::i128::MAX) as usize;
        let non_zero_count = free_digit - zero_count;
        count += 9usize.pow(non_zero_count as u32) * zero_pos_choices; // add number with zero in it as they're always divisible
    }
    for free_sum in free_digit..=9 * free_digit {
        let sum = fixed_sum + free_sum;
        if sum % fixed_prod == 0 {
            let free_prod = sum / fixed_prod;
            count += count_dist(free_digit, free_prod, free_sum, count_dist_mem);
        }
    }
    count
}

struct CountDistMem(HashMap<(usize, usize, usize), usize>);

fn count_dist(free_digit: usize, free_prod: usize, free_sum: usize, mem: &mut CountDistMem) -> usize {
    if free_digit == 0 {
        return if free_sum == 0 && free_prod == 1 {
            1
        } else {
            0
        };
    }
    let key = (free_digit, free_prod, free_sum);
    if let Some(&c) = mem.0.get(&key) {
        return c;
    }
    let mut count = 0;
    for i in 1..9 {
        if free_prod % i == 0 && free_sum >= i {
            count += count_dist(free_digit - 1, free_prod / i, free_sum - i, mem);
        }
    }
    mem.0.insert(key, count);
    count
}

fn solve(sc: &mut Scanner) {
    let a = sc.next::<usize>();
    let b = sc.next::<usize>();
    let mut dist_mem = CountDistMem(HashMap::new());
    
}

fn main() {
    let mut sc = Scanner::new();
    let test_cases = sc.next::<usize>();
    for case_number in 1..=test_cases {
        print!("Case #{}:", case_number);
        solve(&mut sc);
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

mod math {
    const USIZE_BITS: u32 = 64;
    /// O(1)
    pub fn log2_ceil(x: usize) -> u32 {
        if x == 0 {
            0
        } else {
            USIZE_BITS - (x - 1).leading_zeros()
        }
    }
    /// O(1)
    pub fn pow2_ceil(x: usize) -> usize {
        let n = log2_ceil(x);
        2usize.pow(n)
    }
    /// O(1)
    pub fn log2_floor(x: usize) -> u32 {
        if x == 0 {
            0
        } else {
            USIZE_BITS - x.leading_zeros() - 1
        }
    }
    /// O(1)
    pub fn pow2_floor(x: usize) -> usize {
        let n = log2_floor(x);
        2usize.pow(n)
    }

    /// O(1)
    pub fn highest_one_bit(x: usize) -> u32 {
        // assuming 64 bit system
        USIZE_BITS - x.leading_zeros()
    }

    /// O(n)
    pub fn factorial(n: i128, m: i128) -> i128 {
        let mut r: i128 = 1;
        for i in 1..=n {
            r = (r * i.rem_euclid(m)).rem_euclid(m);
        }
        r
    }

    /// O(r)
    pub fn permutations(n: i128, r: i128, m: i128) -> i128 {
        let mut p: i128 = 1;
        for i in (n - r + 1)..=n {
            p = (p * i.rem_euclid(m)).rem_euclid(m);
        }
        p
    }

    /// O(min((n-r)*r, min(n-r, r)^2 * log(n)))
    pub fn binomial(n: i128, r: i128, m: i128) -> i128 {
        if (r - n / 2).abs() < n / 4 || n > std::usize::MAX as i128 || r > std::usize::MAX as i128 {
            multinomial(n, &[r], m)
        } else {
            let r = r as usize;
            let n = n as usize;
            let mut row = vec![1; r as usize + 1];
            for _i in 0..(n - r) {
                for j in 1..=r {
                    row[j] += row[j - 1];
                    row[j] %= m;
                }
            }
            row[r]
        }
    }

    /// O(n * log(n))
    pub fn binomial_mod_inv(n: i128, r: i128, m: i128) -> Option<i128> {
        let r = r.min(n - r);
        let mut c = 1;
        for i in (n - r + 1)..=n {
            c = (c * i.rem_euclid(m)).rem_euclid(m);
        }
        for i in 2..=r {
            c = (c * mod_inv(i, m)?).rem_euclid(m);
        }
        Some(c)
    }

    /// O((n-max(rs))^2 * log(n))
    pub fn multinomial(n: i128, rs: &[i128], m: i128) -> i128 {
        let mut rs = rs.to_vec();
        let sum: i128 = rs.iter().sum();
        if sum != n {
            rs.push(n - sum);
        }
        let mut maxi = 0;
        for i in 1..rs.len() {
            if rs[maxi] < rs[i] {
                maxi = i;
            }
        }
        let max = rs[maxi];
        rs.remove(maxi);
        let mut ns = ((max + 1)..=n).collect::<Vec<_>>();
        while let Some(r) = rs.pop() {
            for d in 2..=r {
                let mut d = d;
                for n in ns.iter_mut() {
                    let g = get_gcd(*n, d);
                    d /= g;
                    *n /= g;
                    if d == 1 {
                        break;
                    }
                }
            }
        }
        ns.iter().fold(1, |res, &n| (res * n).rem_euclid(m))
    }

    /// O(sqrt(x))
    pub fn get_prime_facts(mut x: i128) -> Vec<(i128, usize)> {
        let mut result = Vec::new();
        let mut n = 2;
        while n * n <= x {
            if x % n == 0 {
                x /= n;
                let mut count = 1;
                while x % n == 0 {
                    x /= n;
                    count += 1;
                }
                result.push((n, count));
            }
            n += 1;
        }
        if x != 1 {
            result.push((x, 1));
        }
        result
    }

    /// O(sqrt(x))
    pub fn get_factors(x: i128) -> Vec<i128> {
        let pfs = get_prime_facts(x);
        let mut result = Vec::new();
        fn make_primes(mut fact: i128, pfi: usize, pfs: &[(i128, usize)], result: &mut Vec<i128>) {
            if pfi >= pfs.len() {
                result.push(fact);
                return;
            }
            make_primes(fact, pfi + 1, pfs, result); // choose zero
            for _ in 0..pfs[pfi].1 {
                // choose 1 or more
                fact *= pfs[pfi].0;
                make_primes(fact, pfi + 1, pfs, result);
            }
        }
        make_primes(1, 0, &pfs, &mut result);
        result
    }

    /// O(sqrt(x))
    pub fn get_facts_count(x: i128) -> usize {
        let pfs = get_prime_facts(x);
        let mut r = 1;
        for (_, count) in pfs {
            r *= count + 1;
        }
        r
    }

    /// O(sqrt(x))
    pub fn get_facts_sum(x: i128) -> i128 {
        let pfs = get_prime_facts(x);
        let mut sum = 1;
        for (pf, count) in pfs {
            sum *= (pf.pow(count as u32 + 1) - 1) / (pf - 1);
        }
        sum
    }

    /// O(sqrt(x))
    pub fn get_facts_prod(x: i128) -> i128 {
        x.pow(get_facts_count(x) as u32 / 2)
    }

    /// O(log(x))
    pub fn get_gcd(mut x: i128, mut y: i128) -> i128 {
        while y != 0 {
            let ty = y;
            y = x.rem_euclid(y);
            x = ty;
        }
        x
    }

    /// O(sqrt(x))
    /// i.e. φ
    pub fn get_smaller_coprimes_count(x: i128) -> i128 {
        let pfs = get_prime_facts(x);
        let mut c = 1;
        for (pf, count) in pfs {
            c *= pf.pow(count as u32 - 1) * (pf - 1);
        }
        c
    }

    /// O(log(n))
    pub fn pow(x: i128, n: i128, m: i128) -> i128 {
        let x = x.rem_euclid(m);
        if n == 0 {
            1
        } else if n % 2 == 0 {
            pow(x, n / 2, m).pow(2).rem_euclid(m)
        } else {
            (pow(x, n - 1, m) * x).rem_euclid(m)
        }
    }

    /// O(sqrt(x))
    pub fn mod_inv(x: i128, m: i128) -> Option<i128> {
        if get_gcd(x, m) != 1 {
            return None;
        }
        Some(pow(x, get_smaller_coprimes_count(m) - 1, m))
    }

    /// O(log(x))
    pub fn solve_ax_by_c(a: i128, b: i128, c: i128) -> Option<(i128, i128)> {
        let mut rn = a; // r0 = a
        let mut rn1 = b; // r1 = b
        let mut q = Vec::new();
        let mut r = Vec::new();
        while rn1 != 0 {
            // r[n] = q[n]r[n+1] + r[n+2]
            let qn = rn / rn1;
            let rn2 = rn.rem_euclid(rn1);

            q.push(qn);
            r.push(rn);

            rn = rn1;
            rn1 = rn2;
        }
        let g = rn;

        if c % g != 0 {
            return None;
        }

        // r[n-1] = q[n-1]r[n] + g
        // g = r[n-1] - q[n-1]r[n]
        let mut x = 1;
        let mut y = -q[q.len() - 2];
        for i in (2..r.len()).rev() {
            // g = r[i-1]x + r[i]y
            // r[i-2] = q[i-2]r[i-1] + r[i]
            // r[i] = r[i-2] - q[i-2]r[i-1]
            // g = r[i-1]x + r[i-2]y - q[i-2]r[i-1]y
            // g = (y)r[i-2] + (x - q[i-2]y)r[i-1]
            let new_x = y;
            let new_y = x - (q[i - 2] * y);
            x = new_x;
            y = new_y;
            // g = r[i-2]x + r[i-1]y
        }
        Some((x * (c / g), y * (c / g)))
    }

    /// O(am.len() * sqrt(m_prod))
    pub fn solve_crt(am: &[(i128, i128)]) -> Option<i128> {
        let mut result = 0;
        let m_prod = am.iter().fold(1, |res, &(_, m)| res * m);
        for &(a, m) in am.iter() {
            let x = m_prod / m;
            let inv_x = mod_inv(x, m)?;
            result += a * x * inv_x;
        }
        Some(result)
    }

    /// O(sqrt(x))
    pub fn isqrt(x: i128) -> i128 {
        if x < 0 {
            panic!("negative number doesn't have sqrt");
        }
        if x <= 1 {
            return x;
        }

        let mut x0 = x / 2;
        let mut x1 = (x0 + x / x0) / 2;

        while x1 < x0 {
            x0 = x1;
            x1 = (x0 + x / x0) / 2;
        }
        x0
    }

    #[cfg(test)]
    mod test {
        use std::{collections::HashSet, iter::FromIterator};

        use super::*;

        #[test]
        fn test() {
            assert_eq!(log2_ceil(128), 7);
            assert_eq!(log2_ceil(129), 8);
            assert_eq!(pow2_ceil(128), 128);
            assert_eq!(pow2_ceil(129), 256);
            assert_eq!(log2_floor(128), 7);
            assert_eq!(log2_floor(127), 6);
            assert_eq!(pow2_floor(128), 128);
            assert_eq!(pow2_floor(127), 64);
            assert_eq!(highest_one_bit(128), 8);
            assert_eq!(highest_one_bit(127), 7);
            assert_eq!(factorial(10, 1007), 579);
            assert_eq!(permutations(100, 66, 1_000_007), 188_297);
            assert_eq!(binomial(100, 66, 1_000_007), 754_526);
            assert_eq!(binomial_mod_inv(100, 66, 1_000_007), None);
            assert_eq!(binomial_mod_inv(100, 66, 100_000_007), Some(39_929_235));
            assert_eq!(
                multinomial(100, &(1..=13).collect::<Vec<_>>(), 1_000_007),
                497_843
            );
            assert_eq!(get_prime_facts(4), vec![(2, 2)]);
            assert_eq!(get_prime_facts(12), vec![(2, 2), (3, 1)]);
            assert_eq!(get_prime_facts(84), vec![(2, 2), (3, 1), (7, 1)]);
            assert_eq!(get_prime_facts(17), vec![(17, 1)]);
            assert_eq!(
                get_factors(84).iter().collect::<HashSet<_>>(),
                HashSet::from_iter(&[1, 2, 3, 4, 6, 7, 12, 14, 21, 28, 42, 84])
            );
            assert_eq!(get_facts_count(84), 12);
            assert_eq!(get_facts_sum(1), 1);
            assert_eq!(get_facts_sum(84), 224);
            assert_eq!(get_facts_prod(1), 1);
            assert_eq!(get_facts_prod(84), 351_298_031_616);
            assert_eq!(get_gcd(24, 36), 12);
            assert_eq!(get_smaller_coprimes_count(12), 4);
            assert_eq!(get_smaller_coprimes_count(11), 10);
            assert_eq!(
                pow(123, 123, std::i64::MAX.into()),
                5_600_154_571_973_842_357
            );
            assert_eq!(mod_inv(6, 17), Some(3));
            assert_eq!(mod_inv(6, 9), None);
            assert_eq!(solve_ax_by_c(39, 15, 12), Some((8, -20)));
            assert_eq!(solve_ax_by_c(191, 1097, 12), Some((2688, -468)));
            assert_eq!(solve_ax_by_c(39, 15, 10), None);
            assert_eq!(solve_crt(&[(3, 5), (4, 7), (2, 3)]), Some(263));
            assert_eq!(solve_crt(&[(3, 5), (4, 6), (2, 3)]), None);
            assert_eq!(isqrt(12), 3);
            assert_eq!(isqrt(1024), 32);
        }
    }
}
#[allow(unused_imports)]
use math::*;
