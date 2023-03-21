use std::io::stdin;
use std::collections::HashSet;

const TABLE: [&str; 26] = [
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
    "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
];
pub fn unique_morse_representations(words: Vec<String>) -> i32 {
	words
		.iter()
		.map(|w| w.chars().map(|c| TABLE[c as usize - 'a' as usize]).collect::<String>())
        .collect::<HashSet<_>>()
        .len() as i32
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let words:Vec<String> = line.trim().split(",").map(|c| c.to_string()).collect();
    unique_morse_representations(words);
}
