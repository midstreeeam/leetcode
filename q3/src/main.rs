use std::{io::stdin};

fn length_of_longest_substring(s: &String) -> i32 {
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut ans:i32 = std::cmp::min(s.len() as i32,1);

    while i<s.len(){
        let next_char: &[char] = &[s.as_bytes()[i] as char];
        while s[j..i].contains(next_char){
            j+=1;
            ans = std::cmp::max(ans, (i-j+1) as i32);
        }
        i+=1;
    }
    ans = std::cmp::max(ans, (i-j) as i32);
    return ans;
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("unable to read line");
    buf=buf.trim().to_string();
    println!("{}",length_of_longest_substring(&mut buf));
}
