use chrono::prelude::*;
// let example = "[1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
// [1518-11-01 00:30] falls asleep
// [1518-11-01 00:55] wakes up
// [1518-11-01 23:58] Guard #99 begins shift
// [1518-11-02 00:40] falls asleep
// [1518-11-02 00:50] wakes up
// [1518-11-03 00:05] Guard #10 begins shift
// [1518-11-03 00:24] falls asleep
// [1518-11-03 00:29] wakes up
// [1518-11-04 00:02] Guard #99 begins shift
// [1518-11-04 00:36] falls asleep
// [1518-11-04 00:46] wakes up
// [1518-11-05 00:03] Guard #99 begins shift
// [1518-11-05 00:45] falls asleep
// [1518-11-05 00:55] wakes up";

#[derive(PartialEq, Debug)]
enum Record<'a> {
    NewGuard(NaiveDateTime, &'a str), // id
    WakesUp(NaiveDateTime),
    FallsAsleep(NaiveDateTime),
}

fn parse_record<'a>(input: &'a str) -> Record<'a> {
    let input: Vec<&str> = input.split(|c| c == '[' || c == ']').collect();
    let date = NaiveDateTime::parse_from_str(input[1], "%Y-%m-%d %H:%M").unwrap();
    let content = input[2].trim();
    if content == "falls asleep" {
        Record::FallsAsleep(date)
    } else if content == "wakes up" {
        Record::WakesUp(date)
    } else {
        let (_, id) = content
            .split_whitespace()
            .find(|s| s.starts_with('#'))
            .unwrap()
            .split_at(1);
        Record::NewGuard(date, id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_record() {
        assert_eq!(
            parse_record("[1518-11-01 00:00] Guard #10 begins shift"),
            Record::NewGuard(
                "1518-11-01T00:00:00".parse::<NaiveDateTime>().unwrap(),
                "10"
            )
        );
        assert_eq!(
            parse_record("[1518-11-01 00:05] falls asleep"),
            Record::FallsAsleep("1518-11-01T00:05:00".parse::<NaiveDateTime>().unwrap())
        );
        assert_eq!(
            parse_record("[1518-11-05 00:55] wakes up"),
            Record::WakesUp("1518-11-05T00:55:00".parse::<NaiveDateTime>().unwrap())
        );
    }
}
