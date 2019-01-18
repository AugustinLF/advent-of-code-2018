use std::collections::LinkedList;
use std::fs;
use std::iter::FromIterator;

fn react_polymer_from_string(polymer: &str) -> LinkedList<char> {
    let polymer: LinkedList<char> = LinkedList::from_iter(polymer.chars());
    return polymer_react(polymer);
}

fn polymer_react_once(mut polymer: LinkedList<char>) -> LinkedList<char> {
    let mut reacted_polymer: LinkedList<char> = LinkedList::new();
    while let Some(unit) = polymer.pop_front() {
        match polymer.front() {
            Some(&first_unit) => {
                if first_unit != unit
                    && first_unit.to_ascii_lowercase() == unit.to_ascii_lowercase()
                {
                    polymer.pop_front();
                    reacted_polymer.append(&mut polymer);
                    return reacted_polymer;
                } else {
                    reacted_polymer.push_back(unit);
                }
            }
            None => {
                reacted_polymer.push_back(unit);
                return reacted_polymer;
            }
        }
    }
    return reacted_polymer;
}

fn polymer_react(mut polymer: LinkedList<char>) -> LinkedList<char> {
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
        assert_eq!(react_polymer_from_string("aA"), LinkedList::new());
        assert_eq!(react_polymer_from_string("abBA"), LinkedList::new());
        assert_eq!(
            react_polymer_from_string("ab"),
            LinkedList::from_iter(vec!['a', 'b'])
        );
        assert_eq!(
            react_polymer_from_string("abAB"),
            LinkedList::from_iter(vec!['a', 'b', 'A', 'B'])
        );
        assert_eq!(
            react_polymer_from_string("a"),
            LinkedList::from_iter(vec!['a'])
        );
        assert_eq!(
            react_polymer_from_string("aAa"),
            LinkedList::from_iter(vec!['a'])
        );
        assert_eq!(
            react_polymer_from_string("aabAAB"),
            LinkedList::from_iter(vec!['a', 'a', 'b', 'A', 'A', 'B'])
        );
        assert_eq!(
            react_polymer_from_string("dabAcCaCBAcCcaDA"),
            LinkedList::from_iter(vec!['d', 'a', 'b', 'C', 'B', 'A', 'c', 'a', 'D', 'A'])
        );
    }
}
