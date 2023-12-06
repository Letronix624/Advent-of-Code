#[cfg(feature = "part_two")]
use rayon::prelude::*;

const INPUT: &str = include_str!("input");

struct Mapping {
    mappings: Vec<[u64; 3]>,
}

impl Mapping {
    pub fn from_vec(vector: Vec<[u64; 3]>) -> Self {
        Self { mappings: vector }
    }

    /// Processes the number with the given mapping rules.
    pub fn process(&self, seed: u64) -> u64 {
        for mapping in self.mappings.iter() {
            if seed >= mapping[1] && seed < mapping[1] + mapping[2] {
                return seed - mapping[1] + mapping[0];
            }
        }
        seed
    }
}

struct Almanac {
    #[cfg(not(feature = "part_two"))]
    seeds: Vec<u64>,
    #[cfg(feature = "part_two")]
    seeds: Vec<std::ops::Range<u64>>,
    maps: Vec<Mapping>,
}

impl Almanac {
    /// Makes the Almanac from the official string.
    pub fn from_string(string: &str) -> Option<Self> {
        let mut mappings: Vec<&str> = string.split("\n\n").map(str::trim).collect();

        let seeds: Vec<Option<u64>> = mappings
            .remove(0)
            .strip_prefix("seeds:")?
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().ok())
            .collect();
        let seeds = if seeds.contains(&None) {
            return None;
        } else {
            #[allow(clippy::let_and_return)]
            let seeds = seeds.into_iter().map(Option::unwrap).collect::<Vec<u64>>();
            #[cfg(feature = "part_two")]
            let seeds = {
                let mut result = vec![];
                for i in 0..seeds.len() / 2 {
                    result.push(seeds[i * 2]..seeds[i * 2] + seeds[i * 2 + 1]);
                }
                result
            };
            seeds
        };

        let maps = mappings
            .into_iter()
            .map(|x| {
                let mapping = x.split_once('\n').unwrap().1;
                let numbers = mapping
                    .split('\n')
                    .map(|x| {
                        let x = x
                            .split_ascii_whitespace()
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>();
                        [x[0], x[1], x[2]]
                    })
                    .collect::<Vec<[u64; 3]>>();
                Mapping::from_vec(numbers)
            })
            .collect::<Vec<Mapping>>();

        Some(Self { seeds, maps })
    }

    pub fn lowest_processed_seed(&self) -> u64 {
        let mut lowest = u64::MAX;

        #[cfg(not(feature = "part_two"))]
        for seed in self.seeds.iter() {
            let mut seed = *seed;
            for mapping in self.maps.iter() {
                seed = mapping.process(seed);
            }
            if seed < lowest {
                lowest = seed;
            }
        }
        #[cfg(feature = "part_two")]
        for seed_range in self.seeds.iter() {
            for seed in seed_range.clone() {
                let seed = std::sync::Arc::new(std::sync::Mutex::new(seed));
                self.maps.par_iter().for_each(|mapping| {
                    let mut seed = seed.lock().unwrap();
                    *seed = mapping.process(*seed);
                });
                if *seed.lock().unwrap() < lowest {
                    lowest = *seed.lock().unwrap();
                }
            }
        }
        lowest
    }
}

impl std::fmt::Debug for Almanac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Almanac")
            .field("seeds", &self.seeds)
            .field("maps", &self.maps)
            .finish()
    }
}
impl std::fmt::Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mapping")
            .field("mapping", &self.mappings)
            .finish()
    }
}

pub fn solve() {
    let almanac = Almanac::from_string(INPUT).unwrap();
    println!("{}", almanac.lowest_processed_seed());
}
