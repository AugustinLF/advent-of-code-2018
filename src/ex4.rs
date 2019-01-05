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
            // handle the guard that wakes up after 1am?
            match record.record_type {
                RecordType::NewGuard(id) => {
                    current_guard_id = Some(id);
                    presences.entry(id).or_insert(map);
                    current_asleep_date = Some(record.date);
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

fn get_guards_presence(input: &str) -> u32 {
    let presences: Presences = HashMap::new();
    let mut records: Vec<Record> = input.trim().lines().map(parse_record).collect();
    records.sort_by_key(|record| record.date);
    records.reverse();

    let presence = rec_get_guards_presence(records, presences, None, None);

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
    let mut minutes_slept: HashMap<u32, u8> = HashMap::new();
    let mut sleepiest_minute = (120, 0);
    for &date_time in presence.get(&sleepy_guard_id).unwrap().keys() {
        let minute_count = minutes_slept.entry(date_time.minute()).or_insert(0);
        *minute_count += 1;
        if *minute_count > sleepiest_minute.1 {
            sleepiest_minute = (date_time.minute(), *minute_count);
        }
    }
    let sleepiest_minute = sleepiest_minute.0;
    return sleepiest_minute * u32::from(sleepy_guard_id);
}

pub fn exercise_4_1() -> u32 {
    let content = fs::read_to_string("./inputs/input4").unwrap();
    return get_guards_presence(&content);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_exercise_4_1() {
        assert_eq!(exercise_4_1(), 4716);
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

        assert_eq!(get_guards_presence(example), 240);
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
