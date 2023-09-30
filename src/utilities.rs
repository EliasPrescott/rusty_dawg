pub fn common_prefix(a: &Vec<char>, b: &Vec<char>) -> usize {
    let mut prefix_index = 0;
    for i in 0..a.len().min(b.len()) {
        if a[i] != b[i] {
            break;
        }
        prefix_index += 1;
    }
    prefix_index
}

pub fn prep_word(word: &str) -> Vec<char> {
    word.trim().to_lowercase().chars().collect()
}
