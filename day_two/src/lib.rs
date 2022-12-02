#[derive(PartialEq, Eq, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq)]
enum MatchResult {
    Loss,
    Draw,
    Win,
}

fn determine_result(mine: RockPaperScissors, theirs: RockPaperScissors) -> MatchResult {
    if mine == theirs {
        return MatchResult::Draw;
    }

    if (mine == RockPaperScissors::Rock && theirs == RockPaperScissors::Scissors)
        || (mine == RockPaperScissors::Paper && theirs == RockPaperScissors::Rock)
        || (mine == RockPaperScissors::Scissors && theirs == RockPaperScissors::Paper)
    {
        return MatchResult::Win;
    }

    return MatchResult::Loss;
}

fn score_result(hand: RockPaperScissors, result: MatchResult) -> i32 {
    let mut score = 0;

    if hand == RockPaperScissors::Rock {
        score += 1;
    } else if hand == RockPaperScissors::Paper {
        score += 2;
    } else if hand == RockPaperScissors::Scissors {
        score += 3;
    }

    if result == MatchResult::Draw {
        score += 3;
    } else if result == MatchResult::Win {
        score += 6;
    }

    return score;
}

fn map_hand(hand: &str) -> RockPaperScissors {
    if hand == "A" || hand == "X" {
        return RockPaperScissors::Rock;
    }
    if hand == "B" || hand == "Y" {
        return RockPaperScissors::Paper;
    }
    return RockPaperScissors::Scissors;
}

fn get_my_hand(theirs: RockPaperScissors, choice: &str) -> RockPaperScissors {
    if choice == "Y" {
        return theirs;
    }

    // Lose
    if choice == "X" {
        return match theirs {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper,
        };
    }

    return match theirs {
        RockPaperScissors::Rock => RockPaperScissors::Paper,
        RockPaperScissors::Paper => RockPaperScissors::Scissors,
        RockPaperScissors::Scissors => RockPaperScissors::Rock,
    };
}

pub fn part_one(input: &str) -> i32 {
    let games: Vec<&str> = input.split('\n').collect();
    return games
        .iter()
        .take_while(|x| x.len() > 0)
        .fold(0, |res, &game| {
            let hands: Vec<&str> = game.split(' ').collect();
            let theirs = map_hand(hands[0]);
            let mine = map_hand(hands[1]);

            let result = determine_result(mine.clone(), theirs);

            return res + score_result(mine, result);
        });
}

pub fn part_two(input: &str) -> i32 {
    let games: Vec<&str> = input.split('\n').collect();
    return games
        .iter()
        .take_while(|x| x.len() > 0)
        .fold(0, |res, &game| {
            let hands: Vec<&str> = game.split(' ').collect();
            let theirs = map_hand(hands[0]);
            let mine = get_my_hand(theirs.clone(), hands[1]);

            let result = determine_result(mine.clone(), theirs);

            return res + score_result(mine, result);
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
