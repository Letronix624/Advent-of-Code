const INPUT: &str = include_str!("input");

pub fn solve() {
    // Part 2
    #[cfg(feature = "part_two")]
    let input: Vec<String> = {
        let separators = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ];
        INPUT
            .split_whitespace()
            .map(|x| {
                let rev_x: String = x.chars().rev().collect();
                let mut result = x.to_string();
                let (mut l, mut r) = (false, false);
                for step in 0..x.len() {
                    if !l {
                        for (text, number) in separators {
                            if !x[0..step]
                                .chars()
                                .filter(|x| x.is_ascii_digit())
                                .collect::<Vec<char>>()
                                .is_empty()
                            {
                                l = true;
                                break;
                            }
                            if x[0..step].contains(text) {
                                result = result.replacen(text, number, 1);
                                dbg!("hallo");
                                l = true;
                                break;
                            }
                        }
                    }
                    if !r {
                        for (text, number) in separators {
                            if !rev_x[0..step]
                                .chars()
                                .filter(|x| x.is_ascii_digit())
                                .collect::<Vec<char>>()
                                .is_empty()
                            {
                                r = true;
                                break;
                            }
                            let rev_sepparator = &text.chars().rev().collect::<String>();
                            if rev_x[0..step].contains(rev_sepparator) {
                                result = result
                                    .chars()
                                    .rev()
                                    .collect::<String>()
                                    .replacen(rev_sepparator, number, 1)
                                    .chars()
                                    .rev()
                                    .collect();
                                r = true;
                                break;
                            }
                        }
                    }
                }
                result
            })
            .collect()
    };
    #[cfg(not(feature = "part_two"))]
    let input = INPUT.split_whitespace();

    let result: u32 = input
        .iter()
        .map(|x| {
            let digits = x
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>();
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum();
    println!("{result}");
}
