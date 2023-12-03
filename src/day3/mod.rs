const INPUT: &str = include_str!("input");

pub fn solve() {
    let grid: Vec<(usize, Vec<(usize, char)>)> = INPUT
        .trim()
        .split('\n')
        .map(|x| x.chars().enumerate().collect())
        .enumerate()
        .collect();

    let mut result: u32 = 0;

    #[cfg(not(feature = "part_two"))]
    for (y, row) in grid.iter() {
        let mut was_number = false;

        let mut number = String::new();
        let mut valid = false;

        for (x, char) in row.iter() {
            if char.is_ascii_digit() {
                if !was_number {
                    was_number = true;
                    number += &char.to_string();
                    let mut x = *x;
                    'num: loop {
                        'u: for u in -1i32..=1 {
                            for v in -1i32..=1 {
                                let off_v = *y as i32 + v;
                                let off_u = x as i32 + u;
                                if off_v == grid.len() as i32 || off_v < 0 {
                                    continue;
                                }
                                if off_u == grid[off_v as usize].1.len() as i32 || off_u < 0 {
                                    if u == 1 && valid {
                                        result += number.parse::<u32>().unwrap();
                                        number = String::new();
                                        break 'num;
                                    }
                                    continue 'u;
                                }
                                let char = grid[off_v as usize].1[off_u as usize].1;
                                if !char.is_ascii_digit() && char != '.' {
                                    valid = true;
                                }
                                if u == 1 && v == 0 && char.is_ascii_digit() {
                                    x += 1;
                                    number += &char.to_string();
                                    continue 'num;
                                }
                            }
                        }
                        break;
                    }
                }
            } else {
                if valid {
                    result += number.parse::<u32>().unwrap();
                }
                if !number.is_empty() {
                    // println!("number: {}, valid: {} at x{}, y{}", number, valid, x, y);
                    number = String::new();
                }
                was_number = false;
                valid = false;
            }
        }
    }
    #[cfg(feature = "part_two")]
    for (y, row) in grid.iter() {
        for (x, char) in row.iter() {
            if char == &'*' {
                let mut numbers = vec![];

                let mut middle_up = false;
                let mut middle_down = false;
                let mut up_completed = false;
                let mut down_completed = false;
                'v: for v in -1i32..=1 {
                    for u in [0, -1, 1] {
                        let mut offset = 0;
                        let mut number = String::new();
                        let mut right = false;
                        let mut rightstart = false;
                        loop {
                            let off_v = *y as i32 + v;
                            let off_u = *x as i32 + u + offset;
                            if off_v == grid.len() as i32 || off_v < 0 {
                                continue 'v;
                            }
                            if off_u == grid[off_v as usize].1.len() as i32 || off_u < 0 {
                                break;
                            }
                            let char = grid[off_v as usize].1[off_u as usize].1;
                            if (up_completed && v == -1) || (down_completed && v == 1) {
                                up_completed = false;
                                down_completed = false;
                                continue 'v;
                            }
                            if char.is_ascii_digit() {
                                if u == 0 {
                                    middle_up = v == -1;
                                    middle_down = v == 1;
                                }
                                if right {
                                    offset += 1;
                                    if rightstart {
                                        rightstart = false
                                    } else {
                                        number += &char.to_string();
                                    }
                                } else {
                                    offset -= 1;
                                    number = char.to_string() + &number;
                                }
                            } else {
                                offset = 0;
                                if right {
                                    up_completed = middle_up;
                                    down_completed = middle_down;
                                    break;
                                }
                                right = true;
                                rightstart = true;
                            }
                        }
                        if !number.is_empty() {
                            numbers.push(number.parse::<u32>().unwrap());
                        }
                    }
                }
                dbg!(&numbers);
                if numbers.len() == 2 {
                    result += numbers[0] * numbers[1];
                }
            }
        }
    }

    println!("{result}");
}
