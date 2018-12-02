use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn get_product_checksum(id: &str) -> (i32, i32) {
    let mut doubles = 0;
    let mut triplets = 0;
    let mut occurrences: HashMap<char, i8> = HashMap::new();
    for letter in id.chars() {
        let letter_occurrence = occurrences.entry(letter).or_insert(0);
        *letter_occurrence += 1;
    }

    for value in occurrences.values() {
        if *value == 2 {
            doubles = 1;
        }
        if *value == 3 {
            triplets = 1;
        }
    }

    return (doubles, triplets);
}

fn calculate_checksum(ids: Vec<&str>) -> i32 {
    let val = ids.iter().map(|id| get_product_checksum(id)).fold(
        (0, 0),
        |acc, (doubles, triplets)| {
            return (acc.0 + doubles, acc.1 + triplets);
        },
    );
    return val.0 * val.1;
}

pub fn solve_exercise() -> i32 {
    let mut f = File::open("./inputs/input2").expect("file not found");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");
    let split_ids = content.trim().split_whitespace();
    let mut ids: Vec<&str> = Vec::new();
    for id in split_ids {
        ids.push(id);
    }

    return calculate_checksum(ids);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checksum_with_doubles_and_triplets() {
        assert_eq!((1, 1), get_product_checksum("bababc"))
    }

    #[test]
    fn several_doubles() {
        assert_eq!((1, 0), get_product_checksum("aabcdd"))
    }

    #[test]
    fn checksum_of_list() {
        assert_eq!(
            12,
            calculate_checksum(vec![
                "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
            ])
        )
    }

    #[test]
    fn exercise_1() {
        assert_eq!(5478, solve_exercise())
    }
}
