 
use std::collections::HashSet;

fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut results = HashSet::new();
    let mut new_string : Vec<char> = word.to_lowercase().chars().collect();
    new_string.sort();
    for possible_anagram in possible_anagrams {
        if !(word.to_lowercase() == *possible_anagram.to_lowercase()) {
            let mut posible_anagram_array: Vec<char> = possible_anagram.to_lowercase().chars().collect();
            posible_anagram_array.sort();
            if new_string == posible_anagram_array {
                results.insert(possible_anagram.as_ref());
            }
        }
    }

    results
}

fn main() {
    let word = "solemn";
    let inputs = &["lemons", "cherry", "melons"];

    let anagrams = anagrams_for(word, inputs);
    println!("{:?}", anagrams);
}
