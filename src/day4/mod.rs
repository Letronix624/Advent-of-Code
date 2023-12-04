const INPUT: &str = include_str!("input");

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    /// Parses a Card from a given string with the correct format.
    pub fn from_string(string: &str) -> Option<Self> {
        let (index, numbers) = string.split_once(':')?;

        let id = index.strip_prefix("Card")?.trim().parse::<u32>().ok()? - 1;

        let (winning_numbers, your_numbers) = numbers.split_once('|')?;
        let winning_numbers: Vec<Option<u32>> = winning_numbers
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().ok())
            .collect();
        let winning_numbers: Vec<u32> = if winning_numbers.contains(&None) {
            return None;
        } else {
            winning_numbers.into_iter().map(Option::unwrap).collect()
        };

        let your_numbers: Vec<Option<u32>> = your_numbers
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().ok())
            .collect();
        let your_numbers: Vec<u32> = if your_numbers.contains(&None) {
            return None;
        } else {
            your_numbers.into_iter().map(Option::unwrap).collect()
        };

        Some(Self {
            id,
            winning_numbers,
            your_numbers,
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the number of matches of the card.
    fn matches(&self) -> u32 {
        let numbers: Vec<&u32> = self
            .winning_numbers
            .iter()
            .filter(|x| self.your_numbers.contains(x))
            .collect();
        numbers.len() as u32
    }

    /// Returns the number of points this card has according to the rules.
    pub fn points(&self) -> u32 {
        let numbers = self.matches();
        if numbers == 0 {
            return 0;
        }
        2u32.pow(numbers - 1)
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Card")
            .field("id", &self.id)
            .field("winning_numbers", &self.winning_numbers)
            .field("your_numbers", &self.your_numbers)
            .field("points", &self.points())
            .finish()
    }
}

/// Returns the total amount of cards won with the cards.
fn iterative_cards(cards: &Vec<Card>, card_id: usize) -> u32 {
    if card_id >= cards.len() {
        return 0;
    }

    let mut total_cards = 0;
    let mut stack = vec![(card_id, cards[card_id].matches() as usize)];

    while let Some((current_card_id, mut matches)) = stack.pop() {
        total_cards += 1;
        if matches != 0 {
            matches -= 1;

            for i in current_card_id + 1..=current_card_id + matches + 1 {
                if i < cards.len() {
                    let matches_count = cards[i].matches() as usize;
                    stack.push((i, matches_count));
                }
            }
        }
    }

    total_cards
}

pub fn solve() {
    let cards: Vec<Card> = INPUT
        .trim()
        .split('\n')
        .map(|x| Card::from_string(x).unwrap())
        .collect();

    let mut result = 0;

    for card in 0..cards.len() {
        #[cfg(not(feature = "part_two"))]
        {
            result += cards[card].points();
        }
        #[cfg(feature = "part_two")]
        {
            result += iterative_cards(&cards, card);
        }
    }

    println!("{result}");
}
