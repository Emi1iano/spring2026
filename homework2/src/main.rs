fn most_frequent_word(text: &str) -> (String, usize) {
    let mut max_word: String = String::from("");
    let mut max_count: usize = 0;

    let spliced: Vec<&str> = text.split_whitespace().collect();

    for i in 0..spliced.len(){
        let word = spliced[i];
        let mut count: usize = 1;
        for j in i+1..spliced.len() {
            if word == spliced[j] {
                count += 1;
            }
        }
        if max_count < count {
            max_count = count;
            max_word = String::from(word);
        }
    }
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}