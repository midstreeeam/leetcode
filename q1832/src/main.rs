use std::io::stdin;

struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut sentence = sentence.trim().to_string();
        sentence.retain(|c| !c.is_whitespace());
        let mut chars:Vec<char> = sentence.chars().collect();
        chars.sort();
        chars.dedup();
        if chars.len()==26{
            return true
        }
        false
    }
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("unable to read line");
    let res = Solution::check_if_pangram(line);
    println!("{res}");
}
