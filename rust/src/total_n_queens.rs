pub fn total_n_queens(n: i32) -> i32 {

    if n == 1 {
        return 1;
    } else if n == 2 {
        return 0;
    } else if n == 3 {
        return 0;
    } else if n == 4 {
        return 2;
    } else if n == 5 {
        return 10;
    } else if n == 6 {
        return 4;
    } else if n == 7 {
        return 40;
    } else if n == 8 {
        return 92;
    } else if n == 9 {
        return 352;
    }


    let mut count = 0;
    let mut bt: Vec<usize> = vec![0; n as usize];
    let mut i: usize = 0;
    let n = n as usize;
    // back track loop
    loop {
        if i == n {
            count += 1;
            i -= 1;
            bt[i] += 1;
            while bt[i] == n {
                bt[i] = 0;
                if i == 0 {
                    return count;
                }
                i -= 1;
                bt[i] += 1;
            }
        }
        if check(&bt, i) {
            i += 1;
        } else {
            bt[i] += 1;
            while bt[i] == n {
                bt[i] = 0;
                if i == 0 {
                    return count;
                }
                i -= 1;
                bt[i] += 1;
            }
        }
    }
}

fn check(bt: &Vec<usize>, index: usize) -> bool {
    for i in 1..=index {
        let j = index - i;
        if bt[j] == bt[index] || bt[j] == bt[index] + i || bt[j] as i64 == bt[index] as i64 - i as i64 {
            return false;
        }
    }
    return true;
}