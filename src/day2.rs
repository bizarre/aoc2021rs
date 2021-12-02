use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;


#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|d| Command::from_str(d).unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(commands: &Vec<Command>) -> u32 {
    let mut submarine = Submarine::default();

    for command in commands {
        submarine.process(command);
    }

    submarine.depth * submarine.horizontal
}

#[aoc(day2, part2)]
fn solve_part2(commands: &Vec<Command>) -> u32 {
    let mut submarine = Submarine::default();

    for command in commands {
        submarine.process_with_aim(command);
    }

    submarine.depth * submarine.horizontal
}

#[derive(Default)]
struct Submarine {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

impl Submarine {
    fn process(&mut self, command: &Command) {
        match command {
            Command::Forward(amount) => self.horizontal += amount,
            Command::Up(amount) => self.depth -= amount,
            Command::Down(amount) => self.depth += amount
        }
    }

    fn process_with_aim(&mut self, command: &Command) {
      match command {
          Command::Forward(amount) => (self.horizontal += amount, self.depth += self.aim * amount),
          Command::Up(amount) => (self.aim -= amount, ()),
          Command::Down(amount) => (self.aim += amount, ())
      };
  }
}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
  type Err = std::io::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let parts = string.split(" ").collect::<Vec<&str>>();
        let amount: u32 = parts[1].parse().unwrap();

        match parts[0] {
          "forward" => Ok(Command::Forward(amount)),
          "down" => Ok(Command::Down(amount)),
          "up" => Ok(Command::Up(amount)),
          _ => unreachable!()
      } 
    }
}
