use aoc_runner_derive::{aoc, aoc_generator};

type RadarLogDepth = u16;
type RadarLogDepthSum = u16;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<RadarLogDepth> {
    input.lines().map(|d| d.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(logs: &Vec<RadarLogDepth>) -> u32 {
    let mut previous: Option<RadarLogDepth> = None;
    let mut increase = 0;

    for log in logs {
        if let Some(previous) = previous {
            if log > &previous {
                increase += 1
            }
        }

        previous = Some(*log)
    }

    increase
}

#[aoc(day1, part2)]
fn solve_part2(logs: &Vec<RadarLogDepth>) -> u32 {
    let mut previous: Option<RadarLogDepthSum> = None;
    let mut increase = 0;

    for window in logs.windows(3) {
        let sum: u16 = window.into_iter().sum();

        if let Some(previous) = previous {
            if sum > previous {
                increase += 1
            }
        }

        previous = Some(sum);
    }

    increase
}
