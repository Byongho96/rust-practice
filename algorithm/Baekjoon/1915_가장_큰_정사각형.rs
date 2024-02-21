use std::io::{self, BufRead};

fn solution(n: usize, m: usize, arr: Vec<Vec<u32>>) -> u32 {
    let mut mx = 0;

    // make and init DP
    let mut dp = vec![vec![0; m]; n];

    for i in 0..n {
        dp[i][0] = arr[i][0];
        mx = mx.max(dp[i][0]);
    }

    for j in 0..m {
        dp[0][j] = arr[0][j];
        mx = mx.max(dp[0][j]);
    }

    // fill the DP
    for i in 1..n {
        for j in 1..m {
            if arr[i][j] == 0 {
                continue;
            }
            dp[i][j] = (dp[i - 1][j - 1]).min(dp[i - 1][j]).min(dp[i][j - 1]) + 1;
            mx = mx.max(dp[i][j]);
        }
    }

    mx.pow(2)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let nm: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = nm[0];
    let m = nm[1];

    let arr: Vec<Vec<u32>> = (0..n)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let answer = solution(n, m, arr);
    println!("{}", answer);
}