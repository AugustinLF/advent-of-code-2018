use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Debug)]
enum RecordType {
    NewGuard(u16), // id
    WakesUp,
    FallsAsleep,
}
#[derive(PartialEq, Debug)]
struct Record {
    record_type: RecordType,
    date: NaiveDateTime,
}

fn parse_record<'a>(input: &'a str) -> Record {
    let input: Vec<&str> = input.split(|c| c == '[' || c == ']').collect();
    let date = NaiveDateTime::parse_from_str(input[1], "%Y-%m-%d %H:%M").unwrap();
    let content = input[2].trim();
    if content == "falls asleep" {
        Record {
            record_type: RecordType::FallsAsleep,
            date: date,
        }
    } else if content == "wakes up" {
        Record {
            record_type: RecordType::WakesUp,
            date: date,
        }
    } else {
        let (_, id) = content
            .split_whitespace()
            .find(|s| s.starts_with('#'))
            .unwrap()
            .split_at(1);
        Record {
            record_type: RecordType::NewGuard(id.parse().unwrap()),
            date: date,
        }
    }
}

type GuardPresence = HashMap<NaiveDateTime, bool>;
type Presences = HashMap<u16, GuardPresence>;

fn rec_get_guards_presence<'a>(
    mut records: Vec<Record>,
    mut presences: Presences,
    guard_id: Option<u16>,
    asleep_date: Option<NaiveDateTime>,
) -> Presences {
    let mut current_guard_id = guard_id;
    let mut current_asleep_date = asleep_date;
    let map = HashMap::new();

    match records.pop() {
        Some(record) => {
            match record.record_type {
                RecordType::NewGuard(id) => {
                    current_guard_id = Some(id);
                    presences.entry(id).or_insert(map);
                }
                RecordType::FallsAsleep => {
                    current_asleep_date = Some(record.date);
                }
                RecordType::WakesUp => {
                    let guard_id = current_guard_id.expect("There should be a guard id");
                    let guard_presence = presences
                        .get_mut(&guard_id)
                        .expect("The guard should be present");
                    let mut asleep_date = current_asleep_date.expect("The guard should be asleep");
                    current_asleep_date = None;
                    while asleep_date < record.date {
                        guard_presence.insert(asleep_date, true);
                        asleep_date += Duration::minutes(1);
                    }
                }
            }
            return rec_get_guards_presence(
                records,
                presences,
                current_guard_id,
                current_asleep_date,
            );
        }
        None => {
            return presences;
        }
    }
}

fn get_presence_from_input(input: &str) -> Presences {
    let presences: Presences = HashMap::new();
    let mut records: Vec<Record> = input.trim().lines().map(parse_record).collect();
    records.sort_by_key(|record| record.date);
    records.reverse();

    rec_get_guards_presence(records, presences, None, None)
}

fn get_sleepiest_guard_minute(guard_presence: &GuardPresence) -> u32 {
    let mut minutes_slept: HashMap<u32, u8> = HashMap::new();
    let mut sleepiest_minute = (0, 0);
    for &date_time in guard_presence.keys() {
        let minute_count = minutes_slept.entry(date_time.minute()).or_insert(0);
        *minute_count += 1;
        if *minute_count > sleepiest_minute.1 {
            sleepiest_minute = (date_time.minute(), *minute_count);
        }
    }
    return sleepiest_minute.0;
}

fn get_sleepiest_guards_minute(input: &str) -> u32 {
    let presence = get_presence_from_input(input);
    let guards_asleep_time: HashMap<u16, u16> =
        presence.keys().fold(HashMap::new(), |mut map, id| {
            let guard_presence = presence.get(id).unwrap();
            map.insert(
                *id,
                guard_presence
                    .values()
                    .fold(0, |minutes_slept, &is_sleeping| {
                        if is_sleeping {
                            minutes_slept + 1
                        } else {
                            minutes_slept
                        }
                    }),
            );
            return map;
        });

    let (sleepy_guard_id, _) = guards_asleep_time.iter().fold(
        (0, 0),
        |sleepiest_guard, (&guard_id, &guard_asleep_time)| {
            if guard_asleep_time > sleepiest_guard.1 {
                (guard_id, guard_asleep_time)
            } else {
                sleepiest_guard
            }
        },
    );

    let sleepiest_minute = get_sleepiest_guard_minute(presence.get(&sleepy_guard_id).unwrap());
    return sleepiest_minute * u32::from(sleepy_guard_id);
}

fn get_most_likely_alseep_minute(input: &str) -> u32 {
    let presence = get_presence_from_input(input);
    let minute = presence
        .iter()
        .fold(
            HashMap::new(),
            |mut guard_map, (&guard_id, guard_presence)| {
                guard_map.insert(guard_id, get_sleepiest_guard_minute(guard_presence));
                guard_map
            },
        )
        .iter()
        .fold(
            (0, 0), // (id, minute)
            |(answer_id, answer_minute), (&id, &minute)| {
                if minute > answer_minute {
                    (id, minute)
                } else {
                    (answer_id, answer_minute)
                }
            },
        );
    return u32::from(minute.0) * minute.1;
}

pub fn exercise_4_1() -> u32 {
    let content = fs::read_to_string("./inputs/input4").unwrap();
    return get_sleepiest_guards_minute(&content);
}

pub fn exercise_4_2() -> u32 {
    let content = fs::read_to_string("./inputs/input4").unwrap();
    return get_most_likely_alseep_minute(&content);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_exercise_4_1() {
        assert_eq!(exercise_4_1(), 4716);
    }

    #[test]
    fn solve_exercise_4_2() {
        assert_eq!(exercise_4_2(), 117061);
    }

    #[test]
    fn test_get_sleepiest_minute() {
        let example = "[1518-11-01 00:00] Guard #10 begins shift
        [1518-11-01 00:05] falls asleep
        [1518-11-01 00:25] wakes up
        [1518-11-01 00:30] falls asleep
        [1518-11-01 00:55] wakes up
        [1518-11-01 23:58] Guard #99 begins shift
        [1518-11-02 00:40] falls asleep
        [1518-11-02 00:50] wakes up
        [1518-11-03 00:05] Guard #10 begins shift
        [1518-11-03 00:24] falls asleep
        [1518-11-03 00:29] wakes up
        [1518-11-04 00:02] Guard #99 begins shift
        [1518-11-04 00:36] falls asleep
        [1518-11-04 00:46] wakes up
        [1518-11-05 00:03] Guard #99 begins shift
        [1518-11-05 00:45] falls asleep
        [1518-11-05 00:55] wakes up";

        assert_eq!(get_most_likely_alseep_minute(example), 4455);
    }

    #[test]
    fn test_guard_presence() {
        let example = "[1518-11-01 00:00] Guard #10 begins shift
        [1518-11-01 00:05] falls asleep
        [1518-11-01 00:25] wakes up
        [1518-11-01 00:30] falls asleep
        [1518-11-01 00:55] wakes up
        [1518-11-01 23:58] Guard #99 begins shift
        [1518-11-02 00:40] falls asleep
        [1518-11-02 00:50] wakes up
        [1518-11-03 00:05] Guard #10 begins shift
        [1518-11-03 00:24] falls asleep
        [1518-11-03 00:29] wakes up
        [1518-11-04 00:02] Guard #99 begins shift
        [1518-11-04 00:36] falls asleep
        [1518-11-04 00:46] wakes up
        [1518-11-05 00:03] Guard #99 begins shift
        [1518-11-05 00:45] falls asleep
        [1518-11-05 00:55] wakes up";

        assert_eq!(get_sleepiest_guards_minute(example), 240);
    }

    #[test]
    fn test_parse_record() {
        assert_eq!(
            parse_record("[1518-11-01 00:00] Guard #10 begins shift"),
            Record {
                record_type: RecordType::NewGuard(10),
                date: "1518-11-01T00:00:00".parse::<NaiveDateTime>().unwrap()
            }
        );
        assert_eq!(
            parse_record("[1518-11-01 00:05] falls asleep"),
            Record {
                record_type: RecordType::FallsAsleep,
                date: "1518-11-01T00:05:00".parse::<NaiveDateTime>().unwrap()
            }
        );
        assert_eq!(
            parse_record("[1518-11-05 00:55] wakes up"),
            Record {
                record_type: RecordType::WakesUp,
                date: "1518-11-05T00:55:00".parse::<NaiveDateTime>().unwrap()
            }
        );
    }
}
