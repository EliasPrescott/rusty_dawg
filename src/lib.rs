mod utilities;
mod packed_dawg;

pub use packed_dawg::PackedDawg;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_word() {
        let words = include_str!("test_dictionary.txt").lines().collect();
        let dawg = PackedDawg::from_words(&words);
        for word in words {
            assert!(dawg.has_word(word));
        }
    }

    #[test]
    fn lookup() {
        let words = include_str!("test_dictionary.txt").lines().collect();
        let dawg = PackedDawg::from_words(&words);
        for word in words {
            for i in 0..word.chars().count() {
                assert!(dawg.lookup(&word[0..i]).is_some());
            }
        }
    }

    #[test]
    fn search() {
        let words = include_str!("test_dictionary.txt").lines().collect();
        let dawg = PackedDawg::from_words(&words);
        
        let search_results = dawg.search("cherry", 0);
        assert!(search_results.len() == 1);

        let search_results = dawg.search("appl", 1);
        assert!(search_results.len() == 1);

        let search_results = dawg.search("app", 2);
        assert!(search_results.len() == 1);
        
        let mut search_results = dawg.search("a", 6);
        search_results.sort_by(|(_, a_dist), (_, b_dist)| a_dist.cmp(b_dist));
        assert_eq!(
            vec![
                (String::from("apple"), 4),
                (String::from("banana"), 5),
                (String::from("cherry"), 6),
            ],
            search_results
        );
    }
}
