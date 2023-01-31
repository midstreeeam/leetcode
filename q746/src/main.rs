use std::{cmp::min, io::stdin};

pub fn min_cost_climbing_stairs(cost: &Vec<i32>) -> i32 {
    let mut dp = vec![0; cost.len()];
    for i in 0..cost.len() {
        if i < 2 {
            dp[i] = cost[i]
        } else {
            dp[i] = cost[i]+min(dp[i - 1], dp[i - 2]);
            //println!("{}",dp[i]);
        }
    }
    return min(dp[dp.len() - 1], dp[dp.len() - 2]);
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("unable to read line");
    let cost: Vec<i32> = buf
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("{}", min_cost_climbing_stairs(&cost))
}
