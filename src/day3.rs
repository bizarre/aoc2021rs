use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|d| d.to_owned()).collect()
}

#[aoc(day3, part1)]
fn solve_part1(binary: &Vec<String>) -> u32 {
    let length = binary.len();
    let bitlength = binary[0].len();
    let mut gamma: Vec<u32> = vec![0; bitlength];

    for i in 0..bitlength {
        let ones = binary
            .iter()
            .map(|l| l.chars().nth(i).unwrap().to_digit(2).unwrap())
            .filter(|l| *l == 1)
            .count();

        gamma[i] = if ones > length - ones { 1 } else { 0 }
    }

    let epsilon = gamma
        .clone()
        .into_iter()
        .map(|d| (d == 0) as u32)
        .collect::<Vec<u32>>();

    let gamma = gamma.iter().map(|d| d.to_string()).collect::<String>();
    let epsilon = epsilon.iter().map(|d| d.to_string()).collect::<String>();

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn filter_binary_pool(binary: &Vec<String>, invert: bool) -> Vec<String> {
    let bitlength = binary[0].len();
    let mut pool = binary.clone();

    for i in 0..bitlength {
        let length = pool.len();
        let ones = pool
            .iter()
            .map(|l| l.chars().nth(i).unwrap().to_digit(2).unwrap())
            .filter(|l| *l == 1)
            .count();

        let mut target = if ones >= length - ones { 1 } else { 0 };
        if invert {
            target = (target == 0) as u32
        }

        pool = pool
            .iter()
            .filter(|l| l.chars().nth(i).unwrap().to_digit(10).unwrap() == target)
            .map(|l| l.to_string())
            .collect();

        if pool.len() == 1 {
            break;
        }
    }

    pool
}

#[aoc(day3, part2)]
fn solve_part2(binary: &Vec<String>) -> u32 {
    let oxygen = filter_binary_pool(binary, false);
    let co2 = filter_binary_pool(binary, true);

    let oxygen = oxygen.iter().map(|d| d.to_string()).collect::<String>();
    let co2 = co2.iter().map(|d| d.to_string()).collect::<String>();

    let oxygen = u32::from_str_radix(&oxygen, 2).unwrap();
    let co2 = u32::from_str_radix(&co2, 2).unwrap();

    oxygen * co2
}
