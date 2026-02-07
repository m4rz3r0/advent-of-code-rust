advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut pass: u64 = 0;
    let mut current_rot: i64 = 50;
    for rot in input.lines() {
        if rot.is_empty() {
            continue;
        }
        let clicks: i64 = rot[1..].parse().ok()?;

        current_rot += if rot.chars().next()? == 'R' {
            clicks
        } else {
            -clicks
        };

        if current_rot % 100 == 0 {
            pass += 1;
        }
    }

    Some(pass)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pass: u64 = 0;
    let mut current_rot: i64 = 50;
    for rot in input.lines() {
        if rot.is_empty() {
            continue;
        }
        let clicks: i64 = rot[1..].parse().ok()?;

        if rot.chars().next()? == 'R' {
            let new_pos = current_rot + clicks;
            pass += (new_pos / 100) as u64;
            current_rot = new_pos % 100;
        } else {
            let new_pos = current_rot - clicks;
            pass += ((current_rot - 1).div_euclid(100) - (new_pos - 1).div_euclid(100)) as u64;
            current_rot = new_pos.rem_euclid(100);
        }
    }

    Some(pass)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
