use std::io::stdin;

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
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let mut nm = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = nm.next().unwrap();
    let m = nm.next().unwrap();

    let arr: Vec<Vec<u32>> = (0..n)
    .map(|_| {
        buffer.clear();
        stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        buffer
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
    })
    .collect();

    let answer = solution(n, m, arr);
    println!("{answer}");
}