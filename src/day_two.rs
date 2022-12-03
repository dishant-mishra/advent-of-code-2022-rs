pub fn part_one(input: impl Into<String>) -> i32 {
    input
        .into()
        .lines()
        .map(|scores| {
            if scores == "" {
                return 0;
            }
            let mut choices = scores.split(' ');
            let opp_choice = choices.nth(0).unwrap();
            let your_choice = choices.nth(0).unwrap();

            if your_choice == "X" {
                if opp_choice == "A" {
                    return 3 + 1;
                } else if opp_choice == "B" {
                    return 0 + 1;
                } else {
                    return 6 + 1;
                }
            } else if your_choice == "Y" {
                if opp_choice == "A" {
                    return 6 + 2;
                } else if opp_choice == "B" {
                    return 3 + 2;
                } else {
                    return 0 + 2;
                }
            } else {
                if opp_choice == "A" {
                    return 0 + 3;
                } else if opp_choice == "B" {
                    return 6 + 3;
                } else {
                    return 3 + 3;
                }
            }
        })
        .sum::<i32>()
}

pub fn part_two(input: impl Into<String>) -> i32 {
    input
        .into()
        .lines()
        .map(|scores| {
            if scores == "" {
                return 0;
            }
            let mut choices = scores.split(' ');
            let opp_choice = choices.nth(0).unwrap();
            let strategy = choices.nth(0).unwrap();

            if strategy == "X" {
                if opp_choice == "A" {
                    return 0 + 3;
                } else if opp_choice == "B" {
                    return 0 + 1;
                } else {
                    return 0 + 2;
                }
            } else if strategy == "Y" {
                if opp_choice == "A" {
                    return 3 + 1;
                } else if opp_choice == "B" {
                    return 3 + 2;
                } else {
                    return 3 + 3;
                }
            } else {
                if opp_choice == "A" {
                    return 6 + 2;
                } else if opp_choice == "B" {
                    return 6 + 3;
                } else {
                    return 6 + 1;
                }
            }
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use crate::day_two::{part_one, part_two};

    static INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn day_two_part_one() {
        assert!(part_one(INPUT) == 15);
    }

    #[test]
    fn day_two_part_two() {
        assert!(part_two(INPUT) == 12);
    }
}
