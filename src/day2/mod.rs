const INPUT: &str = include_str!("input");
#[allow(dead_code)]
const RED_CUBES: usize = 12;
#[allow(dead_code)]
const GREEN_CUBES: usize = 13;
#[allow(dead_code)]
const BLUE_CUBES: usize = 14;

struct Game {
    id: usize,
    red_cubes: usize,
    green_cubes: usize,
    blue_cubes: usize,
}

impl Game {
    /// Returns if this game is possible with the number or red, green and blue cubes we have in the bag.
    #[allow(dead_code)]
    pub fn possible(&self) -> bool {
        self.red_cubes <= RED_CUBES
            && self.green_cubes <= GREEN_CUBES
            && self.blue_cubes <= BLUE_CUBES
    }

    /// Returns the game index of this game.
    #[allow(dead_code)]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Makes a Game from a string containing the game data if the format matches.
    pub fn from_string(text: &str) -> Option<Self> {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let (index, game) = text.split_once(':')?;
        let id = index.strip_prefix("Game ")?.parse().ok()?;

        let games = game
            .split(';')
            .map(|x| x.trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>());
        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;

        for game in games {
            for color in game {
                let mut color = color.split_ascii_whitespace();

                let quantity = color.next()?.parse().ok()?;
                let identifier = color.next()?;

                match identifier {
                    "red" => {
                        if quantity > red_cubes {
                            red_cubes = quantity;
                        }
                    }
                    "green" => {
                        if quantity > green_cubes {
                            green_cubes = quantity;
                        }
                    }
                    "blue" => {
                        if quantity > blue_cubes {
                            blue_cubes = quantity;
                        }
                    }
                    _ => return None,
                }
            }
        }

        Some(Self {
            id,
            red_cubes,
            green_cubes,
            blue_cubes,
        })
    }
    #[cfg(feature = "part_two")]
    /// Returns the of the minimum set of cubes of this game.
    pub fn minimum_set_power(&self) -> usize {
        self.red_cubes * self.green_cubes * self.blue_cubes
    }
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game")
            .field("id", &self.id)
            .field("red_cubes", &self.red_cubes)
            .field("green_cubes", &self.green_cubes)
            .field("blue_cubes", &self.blue_cubes)
            .finish()
    }
}

pub fn solve() {
    let games = INPUT
        .trim()
        .split('\n')
        .map(|x| Game::from_string(x).unwrap());
    #[cfg(not(feature = "part_two"))]
    let result: usize = games.filter(|x| x.possible()).map(|x| x.id()).sum();

    #[cfg(feature = "part_two")]
    let result: usize = games.map(|x| x.minimum_set_power()).sum();

    println!("{result}");
}
