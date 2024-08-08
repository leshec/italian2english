use rand::prelude::*;

static WORDS: &str = include_str!("../assets/words.txt");

pub fn get_words() -> String {
    WORDS.to_string()
}

pub fn get_random_word_pair() -> (String, String) {
    let lines = WORDS.lines();
    let n_words = lines.count();
    let random_index = rand::thread_rng().gen_range(0..n_words);
    let word_pair = WORDS.lines().nth(random_index).unwrap();

    let mut words = word_pair.split(",");

    let word1 = words.next().unwrap().to_string();
    let word2 = words.next().unwrap().to_string();

    (word1, word2)
}
