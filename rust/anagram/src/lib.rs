use std::collections::{HashSet, HashMap};

fn make_map(word: &str) -> HashMap<char, u32> {
    word.chars()
        .fold(HashMap::new(), |mut map, ch| {
            *map.entry(ch).or_insert(0) += 1;
            map
        })
}

pub fn anagrams_for<'a>(src: &str, anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = src.to_lowercase();
    let word_map = make_map(&word);
    anagrams.iter()
            .filter(|&x| {
                let anagram = x.to_lowercase();
                anagram != word && make_map(&anagram) == word_map
            })
            .cloned()
            .collect::<HashSet<&str>>()
}
