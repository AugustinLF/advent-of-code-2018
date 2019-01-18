use std::fs;

fn react_polymer_from_string(polymer: &str) -> Vec<char> {
    let polymer: Vec<char> = polymer.chars().collect();
    return polymer_react(polymer);
}

fn polymer_react_once(mut polymer: Vec<char>) -> Vec<char> {
    let mut reacted_polymer: Vec<char> = Vec::new();
    while polymer.len() > 0 {
        let unit = polymer.remove(0);
        match polymer.first() {
            Some(&first_unit) => {
                if first_unit != unit
                    && first_unit.to_ascii_lowercase() == unit.to_ascii_lowercase()
                {
                    polymer.remove(0);
                    reacted_polymer.append(&mut polymer);
                    return reacted_polymer;
                } else {
                    reacted_polymer.push(unit);
                }
            }
            None => {
                reacted_polymer.push(unit);
                return reacted_polymer;
            }
        }
    }
    return reacted_polymer;
}

fn polymer_react(mut polymer: Vec<char>) -> Vec<char> {
    let mut initial_length = polymer.len();
    loop {
        if initial_length % 100 == 0 {
            println!("{}", initial_length);
        }
        let polymer_after_reaction = polymer_react_once(polymer);

        if polymer_after_reaction.len() == initial_length {
            return polymer_after_reaction;
        }
        polymer = polymer_after_reaction;
        initial_length = polymer.len();
    }
}

pub fn exercise_5_1() -> usize {
    let content = fs::read_to_string("./inputs/input5").unwrap();
    let polymer = react_polymer_from_string(content.trim());
    return polymer.len();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_exercise_5_1() {
        assert_eq!(exercise_5_1(), 9822);
    }
    #[test]
    fn test_react_polymer() {
        assert_eq!(react_polymer_from_string("aA"), Vec::new());
        assert_eq!(react_polymer_from_string("abBA"), Vec::new());
        assert_eq!(react_polymer_from_string("ab"), vec!['a', 'b']);
        assert_eq!(react_polymer_from_string("abAB"), vec!['a', 'b', 'A', 'B']);
        assert_eq!(react_polymer_from_string("a"), vec!['a']);
        assert_eq!(react_polymer_from_string("aAa"), vec!['a']);
        assert_eq!(
            react_polymer_from_string("aabAAB"),
            vec!['a', 'a', 'b', 'A', 'A', 'B']
        );
        assert_eq!(
            react_polymer_from_string("dabAcCaCBAcCcaDA"),
            vec!['d', 'a', 'b', 'C', 'B', 'A', 'c', 'a', 'D', 'A']
        );
    }
}
