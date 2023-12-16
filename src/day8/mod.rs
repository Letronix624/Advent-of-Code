use hashbrown::HashMap;
#[cfg(feature = "part_two")]
use num::Integer;

const INPUT: &str = include_str!("input");

pub fn solve() {
    let (instructions, map) = INPUT.split_once("\n\n").unwrap();
    let map = map.trim().split('\n').map(|x| {
        let (identifier, ways) = x.split_once('=').unwrap();
        let ways = ways
            .trim()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();

        (identifier.trim(), ways)
    });
    let map: HashMap<&str, (&str, &str)> = HashMap::from_iter(map);

    let result: u64;

    #[cfg(not(feature = "part_two"))]
    {
        let mut position = map.get_key_value("AAA").unwrap();
        let mut steps: u64 = 0;
        'nav: loop {
            for direction in instructions.chars() {
                steps += 1;
                if direction == 'L' {
                    position = map.get_key_value(position.1 .0).unwrap();
                } else {
                    position = map.get_key_value(position.1 .1).unwrap();
                }
                if position.0 == &"ZZZ" {
                    break 'nav;
                }
            }
        }
        result = steps;
    }
    #[cfg(feature = "part_two")]
    {
        let mut completions = vec![];
        let mut positions: Vec<(&&str, &(&str, &str))> = map
            .iter()
            .filter(|(key, _value)| key.ends_with('A'))
            .collect();
        for position in positions.iter_mut() {
            let mut steps: u64 = 0;
            'nav: loop {
                for direction in instructions.chars() {
                    steps += 1;
                    if direction == 'L' {
                        *position = map.get_key_value(position.1 .0).unwrap();
                    } else {
                        *position = map.get_key_value(position.1 .1).unwrap();
                    }
                }
                if position.0.ends_with('Z') {
                    completions.push(steps);
                    break 'nav;
                }
            }
        }
        result = lcm_of_vector(&completions).unwrap();
    }
    println!("{}", result);
}

#[cfg(feature = "part_two")]
fn lcm_of_vector(numbers: &[u64]) -> Option<u64> {
    if let Some(&first) = numbers.first() {
        Some(numbers.iter().skip(1).fold(first, |acc, &x| acc.lcm(&x)))
    } else {
        None
    }
}
