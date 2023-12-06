const INPUT: &str = include_str!("input");

pub fn solve() {
    let mut input = INPUT.split('\n');
    let time = input
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let distance = input
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let races = time.zip(distance);

    #[cfg(feature = "part_two")]
    let races: Vec<(u64, u64)> = {
        let mut time = String::new();
        let mut distance = String::new();

        for race in races {
            time += &race.0.to_string();
            distance += &race.1.to_string();
        }
        vec![(time.parse().unwrap(), distance.parse().unwrap())]
    };

    let mut record_break_quantity = vec![];

    // Time, Distance
    for race in races {
        let mut record_breaks = 0;
        for time in 0..=race.0 {
            let distance = time * (race.0 - time);
            if distance > race.1 {
                record_breaks += 1;
            }
        }
        record_break_quantity.push(record_breaks);
    }

    let result: u64 = record_break_quantity.iter().product();

    println!("{result}");
}
