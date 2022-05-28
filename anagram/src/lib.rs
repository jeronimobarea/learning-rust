use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_len = word.len();

    let lower_word = word.to_lowercase();
    let mut sorted_word: Vec<char> = lower_word.chars().collect();
    sorted_word.sort_unstable();

    HashSet::from_iter(
        possible_anagrams
            .iter()
            .cloned()
            .filter(|x| x.len() == word_len)
            .filter(|x| {
                let lower_x = x.to_lowercase();
                let mut tmp: Vec<char> = lower_x.chars().collect();
                tmp.sort_unstable();
                tmp == sorted_word && lower_x != lower_word
            }),
    )
}
