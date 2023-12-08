use std::collections::{hash_map::Entry, HashMap};

const INPUT: &str = include_str!("input");

pub fn solve() {
    let input = INPUT.trim().split('\n');

    let mut sorted: Vec<Vec<Hand>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]]; // 7

    for card in input {
        let (card, bid) = card.split_once(' ').unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let mut kinds = HashMap::new();
        let mut chars: Vec<char> = card.chars().collect();
        let mut j = 0;
        // For every J every character
        for kind in chars.iter_mut() {
            #[cfg(feature = "part_two")]
            if *kind == 'J' {
                j += 1;
                continue;
            }
            if let Entry::Vacant(e) = kinds.entry(*kind) {
                e.insert(1);
            } else {
                *kinds.get_mut(kind).unwrap() += 1;
            }
        }
        let mut pairs = [0; 5];
        for value in kinds.values() {
            pairs[value - 1] += 1;
        }
        #[cfg(feature = "part_two")]
        let card_type = if (pairs[4] == 1)
            || (pairs[3] == 1 && j == 1)
            || (pairs[2] == 1 && j == 2)
            || (pairs[1] == 1 && j == 3)
            || (j == 4)
            || j == 5
        {
            Type::FiveOfAKind
        } else if pairs[3] == 1
            || (pairs[2] == 1 && j == 1)
            || (pairs[1] == 1 && j == 2)
            || (j == 3)
            || j == 4
        {
            Type::FourOfAKind
        } else if (pairs[2] == pairs[1] && pairs[1] > 0) || (pairs[1] == 2 && j == 1) {
            Type::FullHouse
        } else if pairs[2] == 1 || (pairs[1] == 1 && j == 1) || (j == 2) || j == 3 {
            Type::ThreeOfAKind
        } else if pairs[1] == 2 {
            Type::TwoPair
        } else if pairs[1] == 1 || (j == 1) || j == 2 {
            Type::OnePair
        } else {
            Type::HighCard
        };
        #[cfg(not(feature = "part_two"))]
        let card_type = if (pairs[4] == 1) {
            Type::FiveOfAKind
        } else if pairs[3] == 1 {
            Type::FourOfAKind
        } else if pairs[2] == pairs[1] && pairs[1] > 0 {
            Type::FullHouse
        } else if pairs[2] == 1 {
            Type::ThreeOfAKind
        } else if pairs[1] == 2 {
            Type::TwoPair
        } else if pairs[1] == 1 {
            Type::OnePair
        } else {
            Type::HighCard
        };
        //

        let hand = Hand {
            cards: card,
            card_type,
            bid,
        };
        sorted[card_type as usize].push(hand);
    }

    let sorted: Vec<Hand> = sorted
        .into_iter()
        .flat_map(|mut x| {
            x.sort_by_key(|x| {
                x.cards
                    .chars()
                    .map(|x| {
                        #[cfg(not(feature = "part_two"))]
                        match x {
                            '2' => "00",
                            '3' => "01",
                            '4' => "02",
                            '5' => "03",
                            '6' => "04",
                            '7' => "05",
                            '8' => "06",
                            '9' => "07",
                            'T' => "08",
                            'J' => "09",
                            'Q' => "10",
                            'K' => "11",
                            'A' => "12",
                            _ => {
                                panic!("Invalid character in input file")
                            }
                        }
                        #[cfg(feature = "part_two")]
                        match x {
                            'J' => "00",
                            '2' => "01",
                            '3' => "02",
                            '4' => "03",
                            '5' => "04",
                            '6' => "05",
                            '7' => "06",
                            '8' => "07",
                            '9' => "08",
                            'T' => "09",
                            'Q' => "10",
                            'K' => "11",
                            'A' => "12",
                            _ => {
                                panic!("Invalid character in input file")
                            }
                        }
                    })
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            });
            x
        })
        .collect();
    let result = sorted
        .into_iter()
        .enumerate()
        .map(|x| x.1.bid * (x.0 + 1) as u32)
        .sum::<u32>();

    println!("{result}");
}

#[derive(Debug, Clone, Copy)]
struct Hand<'a> {
    cards: &'a str,
    #[allow(dead_code)]
    card_type: Type,
    bid: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Type {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
