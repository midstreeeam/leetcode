use std::{io::stdin, collections::VecDeque};

struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut gq = VecDeque::from(g);
        let mut sq = VecDeque::from(s);
        let mut res:i32 = 0;
        
        while !(gq.is_empty() || sq.is_empty()){
            if gq[0]<=sq[0]{
                gq.pop_front();
                res += 1;
            }
            sq.pop_front();
        }
        res
    }
}

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();
    stdin().read_line(&mut line1).unwrap();
    stdin().read_line(&mut line2).unwrap();
    let g:Vec<i32> = line1.trim()
                        .split(" ")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();
    let s:Vec<i32> = line2.trim()
                    .split(" ")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
    let res = Solution::find_content_children(g, s);
    println!("{res}");
}
