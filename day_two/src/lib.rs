#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum MatchResult {
    Loss,
    Draw,
    Win,
}

fn map_hand(choice: char) -> Option<Hand> {
    const HANDS: [Hand; 3] = [Hand::Rock, Hand::Paper, Hand::Scissors];

    let decimal_utf8 = choice as u32;
    return match decimal_utf8 {
        // A to C (utf-8)
        65..=67 => Some(HANDS[(decimal_utf8 - 65) as usize]),
        // X to Z (utf-8)
        88..=90 => Some(HANDS[(decimal_utf8 - 88) as usize]),
        _ => None,
    };
}

fn determine_winner(mine: Hand, theirs: Hand) -> MatchResult {
    if mine == theirs {
        return MatchResult::Draw;
    }

    if (mine == Hand::Rock && theirs == Hand::Scissors)
        || (mine == Hand::Paper && theirs == Hand::Rock)
        || (mine == Hand::Scissors && theirs == Hand::Paper)
    {
        return MatchResult::Win;
    }

    return MatchResult::Loss;
}

fn get_desired_result(choice: char) -> Option<MatchResult> {
    const RESULTS: [MatchResult; 3] = [MatchResult::Loss, MatchResult::Draw, MatchResult::Win];

    let decimal_utf8 = choice as u32;
    return match decimal_utf8 {
        88..=90 => Some(RESULTS[(decimal_utf8 - 88) as usize]),
        _ => None,
    };
}

fn get_hand_from_result(theirs: Hand, result: MatchResult) -> Hand {
    if result == MatchResult::Draw {
        return theirs;
    }

    if result == MatchResult::Win {
        return match theirs {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        };
    }

    return match theirs {
        Hand::Rock => Hand::Scissors,
        Hand::Paper => Hand::Rock,
        Hand::Scissors => Hand::Paper,
    };
}

fn score_game(choice: Hand, result: MatchResult) -> i32 {
    return choice as i32 + 1 + result as i32 * 3;
}

pub fn part_one(input: &str) -> i32 {
    let games: Vec<&str> = input.split('\n').collect();
    return games.iter().filter(|x| x.len() > 0).fold(0, |res, &game| {
        let hands: Vec<Hand> = game.chars().filter_map(map_hand).collect();
        return res + score_game(hands[1], determine_winner(hands[1], hands[0]));
    });
}

pub fn part_two(input: &str) -> i32 {
    let raw_input: Vec<&str> = input.split('\n').collect();
    let games: Vec<char> = raw_input
        .iter()
        .take_while(|i| i.len() > 0)
        .flat_map(|s| s.chars().filter(|&c| c != ' '))
        .collect();

    return games
        .iter()
        .step_by(2)
        .zip(games.iter().skip(1).step_by(2))
        .fold(0, |res, (&their_choice, &my_result)| {
            let theirs = map_hand(their_choice).unwrap();
            let result = get_desired_result(my_result).unwrap();
            let mine = get_hand_from_result(theirs, result);
            return res + score_game(mine, result);
        });
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn part_one_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(14264, part_one(input.as_str()));
    }

    #[test]
    fn part_two_returns_correct_result() {
        let input = fs::read_to_string("input.txt").expect("File input.txt should exist");
        assert_eq!(12382, part_two(input.as_str()));
    }
}
