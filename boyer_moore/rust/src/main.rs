use std::cmp;

const CHARACTER_SPACE: usize = 256;

fn create_shift_table(pattern: &Vec<char>) -> Vec<u32> {
    let mut shift_table = vec![0; CHARACTER_SPACE];

    for (idx, character) in pattern.iter().enumerate() {
        shift_table[*character as usize] = idx as u32;
    }

    return shift_table;
}

fn boyer_moore(search_space: &str, pattern: &str) -> Option<usize> {
    let search_space: Vec<char> = search_space.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();

    let space_space_length = search_space.len();
    let pattern_length = pattern.len();
    let max_shift_index = space_space_length - pattern_length; 

    let shift_table = create_shift_table(&pattern);
    
    let mut current_shift_index = 0;
    while current_shift_index <= max_shift_index {
        let mut pattern_index = pattern_length - 1;

        while pattern[pattern_index] == search_space[current_shift_index + pattern_index] {
            if pattern_index == 0 {
                return Some(current_shift_index);
            }

            pattern_index -= 1;
        }

        let mismatch_character_index = search_space[current_shift_index + pattern_index] as usize;
        current_shift_index += cmp::max(1, pattern_index as i32 - shift_table[mismatch_character_index] as i32) as usize;
    }

    return None;
}

fn main() {
    // Input 
    let search_space = "Super Duper Cupcakes";
    let search_term =  "Cupcakes";
    // Input 

    let position_of_search_term = boyer_moore(search_space, search_term);
    if let Some(position) = position_of_search_term {
        println!("Search term found at position: {}", position);
    } else {
        println!("Search term not found in search space");
    }
}