use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    for possible_word in possible_anagrams {
        if is_anagram(word, possible_word) {
            result.insert(possible_word);
        }
    }
    result
}

pub fn is_anagram(first_word: &str, second_word: &str) -> bool {
    let lower_first_word = first_word.to_lowercase();
    let lower_second_word = second_word.to_lowercase();
    if lower_first_word == lower_second_word {
        return false;
    }
    let chars_first_word: Vec<char> = order_chars_in_string(lower_first_word);
    let chars_second_word: Vec<char> = order_chars_in_string(lower_second_word);

    chars_first_word == chars_second_word
}

pub fn order_chars_in_string(first_word: String) -> Vec<char> {
    let mut chars_first_word: Vec<char> = first_word.chars().collect();

    chars_first_word.sort();
    chars_first_word
}
