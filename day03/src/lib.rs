use std::collections::HashSet;

pub fn part_one(input: &str) -> i32 {
    let rucksacks: Vec<&str> = input.split('\n').collect();
    let compartments: Vec<(_, _)> = rucksacks
        .iter()
        .filter(|x| x.len() > 0)
        .map(|ruck| ruck.split_at(ruck.len() / 2))
        .collect();

    let mut sum: i32 = 0;

    for (comp_a, comp_b) in compartments {
        let uniq_a: HashSet<_> = comp_a.chars().collect();
        let uniq_b: HashSet<_> = comp_b.chars().collect();

        let uniq: Vec<_> = uniq_a.intersection(&uniq_b).collect();
        assert_eq!(uniq.len(), 1);

        let decimal_utf8 = *uniq[0] as u32;
        match decimal_utf8 {
            // A-Z (uppercase)
            65..=90 => sum += (decimal_utf8 - 65 + 27) as i32,
            // a-z (lowercase)
            97..=122 => sum += (decimal_utf8 - 97 + 1) as i32,
            _ => unreachable!(),
        }
    }

    return sum;
}

pub fn part_two(input: &str) -> i32 {
    let rucksacks: Vec<&str> = input.split('\n').collect();
    return rucksacks
        .chunks(3)
        .map(|group| {
            return group
                .iter()
                .map(|ruck| ruck.chars().collect::<HashSet<_>>())
                .reduce(|res, ruck| res.intersection(&ruck).map(|c| *c).collect());
        })
        .filter_map(|opt| {
            return match opt {
                Some(id) => Some(id.iter().map(|c| *c).collect::<Vec<_>>()),
                _ => None,
            };
        })
        .filter(|x| x.len() > 0)
        .fold(0, |res, id| {
            let decimal_utf8 = id[0] as u32;
            return match decimal_utf8 {
                // A-Z (uppercase)
                65..=90 => res + (decimal_utf8 - 65 + 27) as i32,
                // a-z (lowercase)
                97..=122 => res + (decimal_utf8 - 97 + 1) as i32,
                _ => unreachable!(),
            };
        });
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn part_one_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(7746, part_one(input.as_str()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(2604, part_two(input.as_str()));
    }
}
