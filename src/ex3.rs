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
        .unwrap()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_claim() {
        assert_eq!(
            Claim {
                id: "123",
                coordinates: (3, 2),
                size: (5, 4)
            },
            parse_claim("#123 @ 3,2: 5x4")
        )
    }
}
