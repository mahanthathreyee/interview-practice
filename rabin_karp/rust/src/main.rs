const PRIME: i32 = 101;
const HASH_BASE: i32 = 256;

struct RabinKarpHash {
    source: Vec<char>,

    hash: i32,
    hash_start: usize,
    hash_end: usize,
    hash_length: usize,

    highest_power: i32
}


impl RabinKarpHash {
    pub fn new(word: &str, word_length: &usize) -> Self {
        let mut word_hash: i32 = 0;
        let source_word: Vec<char> = word.chars().collect();
        let mut highest_power = 1;

        for _ in 0..(*word_length - 1) {
            highest_power = (highest_power * HASH_BASE).rem_euclid(PRIME);
        }

        for character in source_word.iter().take(*word_length) {
            word_hash = ((word_hash * HASH_BASE) + character_index(character)).rem_euclid(PRIME);
        }

        RabinKarpHash {
            source: source_word,

            hash: word_hash,
            hash_start: 0,
            hash_end: *word_length,
            hash_length: *word_length,

            highest_power: highest_power
        }
    }

    fn move_window(&mut self) {
        if self.hash_end < self.source.len() {
            let start_character = self.source[self.hash_start];
            let new_character = self.source[self.hash_end];

            self.hash -= (character_index(&start_character) * self.highest_power).rem_euclid(PRIME);
            self.hash = (self.hash * HASH_BASE).rem_euclid(PRIME);
            self.hash = (self.hash + character_index(&new_character)).rem_euclid(PRIME);

            self.hash_start += 1;
            self.hash_end += 1;
        }
    }

    fn compare(&self, text_hash: &RabinKarpHash) -> bool {
        if self.hash == text_hash.hash {
            let search_term = self.hash_text();
            let source_text = text_hash.hash_text();

            for i in 0..self.hash_length {
                if search_term[i] != source_text[i] {
                    return false;
                }
            }

            return true;
        }

        return false;
    }

    fn hash_text(&self) -> &[char] {
        &self.source[self.hash_start..self.hash_end]
    }
}

/* Utility Functions */
fn character_index(character: &char) -> i32 {
    *character as i32
}
/* Utility Functions */

fn main() {
    // Input 
    let search_term = "Cupcakes";
    let text = "Super Duper Cupcakes";
    // Input 
    
    let word_length = search_term.len();
    let text_length = text.len();

    let search_term_hash = RabinKarpHash::new(search_term, &word_length);
    let mut text_hash = RabinKarpHash::new(text, &word_length);
    println!("     Keyword: Hash = {} Text = {:?}", search_term_hash.hash, search_term_hash.hash_text());

    let range = text_length - word_length + 1;
    for idx in 0..range {
        println!("Search Space: Hash = {} Text = {:?}", text_hash.hash, text_hash.hash_text());
        if search_term_hash.compare(&text_hash) {
            println!("Search term found at: {}", idx);
            break;
        }

        text_hash.move_window();
    }
}
