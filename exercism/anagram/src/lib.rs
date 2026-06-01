use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut results = HashSet::new();
    
    let word_lower = word.to_lowercase(); 
    
    let mut word_sorted: Vec<char> = word_lower.chars().collect();
    word_sorted.sort();

    for possible_anagram in possible_anagrams {
        let candidate_lower = possible_anagram.to_lowercase();
        
        if word_lower != candidate_lower {
            let mut candidate_sorted: Vec<char> = candidate_lower.chars().collect();
            candidate_sorted.sort();
            
            if word_sorted == candidate_sorted {
                results.insert(*possible_anagram);
            }
        }
    }

    results
}
