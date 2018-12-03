use std::collections::HashMap;
use std::fs;

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

fn calculate_checksum(ids: Vec<String>) -> i32 {
    let val =
        ids.iter()
            .map(|id| get_product_checksum(id))
            .fold((0, 0), |acc, (doubles, triplets)| {
                return (acc.0 + doubles, acc.1 + triplets);
            });
    return val.0 * val.1;
}

fn are_ids_similar<'a>(first: &'a str, second: &'a str) -> Option<String> {
    let mut first = first.chars();
    let mut second = second.chars();
    let mut differences = 0;
    let mut matching_id = String::new();

    while let Some(first_char) = first.next() {
        // The two strings should be the same length
        let second_char = second.next().unwrap();
        if first_char != second_char {
            differences += 1;
        } else {
            matching_id.push(first_char);
        }
    }
    if differences > 1 {
        None
    } else {
        Some(matching_id)
    }
}

fn get_ids() -> Vec<String> {
    let content = fs::read_to_string("./inputs/input2").unwrap();
    let split_ids = content.trim().split_whitespace();
    let mut ids = Vec::new();
    for id in split_ids {
        ids.push(id.to_string());
    }
    return ids;
}

fn find_base_id(mut ids: Vec<String>) -> String {
    let el = ids.pop().unwrap();

    let mut new_ids: Vec<String> = Vec::new();
    for id in ids {
        if let Some(matching_id) = are_ids_similar(&id, &el) {
            return matching_id;
        } else {
            new_ids.push(id);
        }
    }
    return find_base_id(new_ids);
}

pub fn solve_exercise_1() -> i32 {
    let ids = get_ids();

    return calculate_checksum(ids);
}

pub fn solve_exercise_2() -> String {
    let ids = get_ids();

    return find_base_id(ids);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_ids_should_work() {
        assert_eq!(
            Some(String::from("fgij")),
            are_ids_similar("fghij", "fguij")
        );
        assert_eq!(None, are_ids_similar("aghij", "fguij"));
    }

    #[test]
    fn checksum_with_doubles_and_triplets() {
        assert_eq!((1, 1), get_product_checksum("bababc"))
    }

    #[test]
    fn several_doubles() {
        assert_eq!((1, 0), get_product_checksum("aabcdd"))
    }

    #[test]
    fn similar_ids() {
        assert_eq!(
            "fgij",
            find_base_id(vec![
                String::from("abcde"),
                String::from("fghij"),
                String::from("klmno"),
                String::from("pqrst"),
                String::from("fguij"),
                String::from("axcye"),
                String::from("wvxyz"),
            ])
        )
    }

    #[test]
    fn checksum_of_list() {
        assert_eq!(
            12,
            calculate_checksum(vec![
                String::from("abcdef"),
                String::from("bababc"),
                String::from("abbcde"),
                String::from("abcccd"),
                String::from("aabcdd"),
                String::from("abcdee"),
                String::from("ababab"),
            ])
        )
    }

    #[test]
    fn exercise_2_1() {
        assert_eq!(5478, solve_exercise_1())
    }

    #[test]
    fn exercise_2_2() {
        assert_eq!(
            String::from("qyzphxoiseldjrntfygvdmanu"),
            solve_exercise_2()
        )
    }
}
