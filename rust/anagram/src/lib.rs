use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    HashSet::new()
}

pub fn is_anagram(firstWord: &str, secondWord: &str) -> bool {
    let charsFirstWord: Vec<char> = firstWord.chars().collect();
    let charsSecondtWord: Vec<char> = secondWord.chars().collect();

    true
}

pub fn rr(firstWord: &str) -> String {
    let mut charsFirstWord: Vec<char> = firstWord.chars().collect();
    // let charsSecondtWord: Vec<char> = secondWord.chars().collect();

    charsFirstWord.sort();
    charsFirstWord.into_iter().collect()
}