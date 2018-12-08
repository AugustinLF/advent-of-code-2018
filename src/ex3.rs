use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Claim<'a> {
    id: &'a str,
    coordinates: (u16, u16),
    size: (u16, u16),
}
impl<'a> PartialEq for Claim<'a> {
    fn eq(&self, other: &Claim) -> bool {
        self.id == other.id && self.coordinates == other.coordinates && self.size == other.size
    }
}

fn parse_claim<'a>(claim: &'a str) -> Claim<'a> {
    let mut split_str = claim.split_whitespace();

    let (_, id) = split_str.next().unwrap().split_at(1);

    // drop the @
    split_str.next();

    let mut coordinates = split_str
        .next()
        .expect(claim)
        .split(":")
        .next()
        .unwrap()
        .split(",");
    let x = coordinates.next().unwrap().parse().unwrap();
    let y = coordinates.next().unwrap().parse().unwrap();

    let mut size = split_str.next().unwrap().split("x");
    let width = size.next().unwrap().parse().unwrap();
    let height = size.next().unwrap().parse().unwrap();

    Claim {
        id,
        coordinates: (x, y),
        size: (width, height),
    }
}

fn get_covered_fabric(claim: Claim, mut fabric: HashMap<String, u16>) -> HashMap<String, u16> {
    let mut x = claim.coordinates.0;
    while x < claim.coordinates.0 + claim.size.0 {
        let mut y = claim.coordinates.1;
        while y < claim.coordinates.1 + claim.size.1 {
            let key = x.to_string() + "," + &y.to_string();
            let entry = fabric.entry(key).or_insert(0);
            *entry += 1;
            y += 1;
        }
        x += 1;
    }
    return fabric;
}

fn get_covered_fabric_for_claims(
    mut claims: Vec<Claim>,
    fabric: HashMap<String, u16>,
) -> HashMap<String, u16> {
    return match claims.pop() {
        Some(claim) => get_covered_fabric_for_claims(claims, get_covered_fabric(claim, fabric)),
        None => fabric,
    };
}

fn find_fabric_for_claims(claims: Vec<Claim>) -> u32 {
    let fabric = get_covered_fabric_for_claims(claims, HashMap::new());
    let mut count = 0;
    for (_, &times_covered) in fabric.iter() {
        if times_covered >= 2 {
            count += 1;
        }
    }
    return count;
}

pub fn solve_exercise_1() -> u32 {
    let content = fs::read_to_string("./inputs/input3").unwrap();
    let split_claims = content.trim().lines();
    let mut claims = Vec::new();
    for claim in split_claims {
        claims.push(claim.to_string());
    }
    let claims: Vec<Claim> = claims.iter().map(|claim| parse_claim(claim)).collect();

    return find_fabric_for_claims(claims);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_exercise_3_1() {
        assert_eq!(107043, solve_exercise_1())
    }

    #[test]
    fn test_fabric_for_claims() {
        assert_eq!(
            4,
            find_fabric_for_claims(vec![
                Claim {
                    id: "1",
                    coordinates: (1, 3),
                    size: (4, 4)
                },
                Claim {
                    id: "2",
                    coordinates: (3, 1),
                    size: (4, 4)
                },
                Claim {
                    id: "2",
                    coordinates: (5, 5),
                    size: (2, 2)
                },
            ])
        )
    }
    #[test]
    fn test_parse_claim() {
        assert_eq!(
            Claim {
                id: "123",
                coordinates: (3, 2),
                size: (5, 4)
            },
            parse_claim("#123 @ 3,2: 5x4")
        );
    }

    #[test]
    fn covered_fabric() {
        let mut map = HashMap::new();
        map.insert(String::from("3,2"), 1);
        map.insert(String::from("3,3"), 1);
        map.insert(String::from("4,2"), 1);
        map.insert(String::from("5,2"), 1);
        map.insert(String::from("4,3"), 1);
        map.insert(String::from("5,3"), 1);
        assert_eq!(
            map,
            get_covered_fabric(
                Claim {
                    id: "123",
                    coordinates: (3, 2),
                    size: (3, 2)
                },
                HashMap::new()
            )
        )
    }
}
