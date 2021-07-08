use std::cmp::min;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1: &[u8] = word1.as_bytes();
    let word2: &[u8] = word2.as_bytes();
    let mut dp: Vec<Vec<i32>> = Vec::with_capacity(word1.len());
    dp.push(Vec::with_capacity(word2.len()));
    for j in 0..word2.len() {
        dp[0].push(j as i32);
    }
    for i in 1..word1.len() {
        dp.push(Vec::with_capacity(word2.len()));
        let s = dp[i-1][0] + 1;
        dp[i].push(s);
        for j in 1..word2.len() {
            let l1 = dp[i][j-1] + 1;
            let l2 = dp[i-1][j] + 1;
            let mut l3 = dp[i-1][j-1];
            if word1[i] != word2[j] {
                l3 += 1;
            }
            dp[i].push(min(l1, min(l2, l3)));
        }
    }
    return dp[word1.len() - 1][word2.len() - 1];
}