use std::collections::HashSet;

fn sort_string(string: &str) -> String {
    // Split the chars and make them lowercase, and then put them in a vector to be sorted 
    let mut word_vec = string.to_lowercase().chars().collect::<Vec<char>>();
    word_vec.sort_unstable();
    let string_sorted = word_vec.iter().collect::<String>();
    string_sorted
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Split the chars and make them lowercase, and then put them in a vector to be sorted 
    let word_sorted = sort_string(word);

    let mut anagrams: HashSet<&str> = HashSet::new();

    for anagram in possible_anagrams{
        let anagram_sorted =  sort_string(anagram);
        if anagram_sorted == word_sorted && word.to_lowercase() != anagram.to_lowercase(){
            anagrams.insert(anagram);
        }
    }

    anagrams
}
