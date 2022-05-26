use std::collections::{HashMap, HashSet};

/*
* 1. Go through the map of possible anagram
* 2. Check length of the possible anagram
* TODO: 3. Lowercase the word and the anagram
* 4.
*/
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut word_letters = HashMap::new();
    for c in word.chars() {
        word_letters.insert(c.to_string().to_lowercase(), 0);
    }

    let mut result: HashSet<&'a str> = HashSet::new();
    for anagram in possible_anagrams.iter() {
        let mut counter = 0;
        for c in word.chars() {
            if word.len() != anagram.len() {
                continue;
            }
            match word_letters.get(&c.to_string()) {
                Some(_) => counter += 1,
                None => counter -= 1,
            }
            if counter < 0 {
                continue;
            }
        }

        if counter > 0 {
            result.insert(anagram);
        }
    }

    return result;
}
