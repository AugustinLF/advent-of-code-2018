use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn generate_frequency(initial_frequency: i32, mut input: Vec<i32>) -> i32 {
    let elem = input.pop();
    return match elem {
        Some(value) => generate_frequency(initial_frequency + value, input),
        None => initial_frequency,
    };
}

fn from_string_to_frequencies(content: String) -> Vec<i32> {
    let mut input: Vec<i32> = Vec::new();
    for freq in content.split_whitespace() {
        if let Ok(freq_number) = freq.parse::<i32>() {
            input.push(freq_number);
        }
    }
    return input;
}

pub fn solve_exercise(process: fn(i32, Vec<i32>) -> i32) -> i32 {
    let mut f = File::open("./inputs/input1").expect("file not found");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");

    let input = from_string_to_frequencies(content);

    return process(0, input);
}

fn find_repeated_frequency(base_frequency: i32, input: Vec<i32>) -> i32 {
    // My naive implementation used a Vec. 360s -> 2s
    let mut found_frequencies: HashMap<i32, bool> = HashMap::new();
    let mut current_frequency = base_frequency;
    found_frequencies.insert(current_frequency, true);
    loop {
        for frequency in &input {
            current_frequency += frequency;
            if found_frequencies.contains_key(&current_frequency) {
                return current_frequency;
            } else {
                found_frequencies.insert(current_frequency, true);
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_frequency_adds() {
        assert_eq!(4, generate_frequency(0, vec![-1, 2, 3]));
        assert_eq!(3, generate_frequency(0, vec![1, -2, 3, 1]));
    }

    #[test]
    fn find_repeated_frequency_works() {
        assert_eq!(0, find_repeated_frequency(0, vec![1, -1]));
        assert_eq!(10, find_repeated_frequency(0, vec![3, 3, 4, -2, -4]));
    }

    #[test]
    fn exercise_1() {
        assert_eq!(574, solve_exercise(generate_frequency));
    }

    #[test]
    fn exercise_2() {
        assert_eq!(452, solve_exercise(find_repeated_frequency));
    }
}
