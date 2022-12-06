use std::collections::HashSet;

pub fn part_one(input: &str) -> i32 {
    return input
        .lines()
        .filter(|x| x.len() > 0)
        .map(|ruck| ruck.split_at(ruck.len() / 2))
        .map(|(comp_a, comp_b)| {
            *comp_a
                .chars()
                .collect::<HashSet<_>>()
                .intersection(&comp_b.chars().collect::<HashSet<_>>())
                .nth(0)
                .unwrap() as u32
        })
        .fold(0, |res, uniq| {
            return match uniq {
                // A-Z (uppercase)
                65..=90 => res + uniq - 65 + 27,
                // a-z (lowercase)
                97..=122 => res + uniq - 97 + 1,
                _ => res,
            };
        }) as i32;
}

pub fn part_one_imperative(input: &str) -> i32 {
    let rucksacks: Vec<&str> = input.split('\n').collect();
    let mut sum: u32 = 0;
    let mut uniq_a: HashSet<char> = HashSet::new();
    let mut uniq_b: HashSet<char> = HashSet::new();

    for ruck in rucksacks {
        if ruck.len() == 0 {
            continue;
        }

        uniq_a.clear();
        uniq_b.clear();

        let (comp_a, comp_b) = ruck.split_at(ruck.len() / 2);
        for a in comp_a.chars() {
            uniq_a.insert(a);
        }
        for b in comp_b.chars() {
            uniq_b.insert(b);
        }

        let uniq = *uniq_a.intersection(&uniq_b).nth(0).unwrap() as u32;

        match uniq {
            // A-Z (uppercase)
            65..=90 => sum += uniq - 65 + 27,
            // a-z (lowercase)
            97..=122 => sum += uniq - 97 + 1,
            _ => unreachable!(),
        }
    }

    return sum as i32;
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
    fn part_one_imperative_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(7746, part_one_imperative(input.as_str()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(2604, part_two(input.as_str()));
    }
}
