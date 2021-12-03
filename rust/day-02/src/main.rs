use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

// Helpers
// - Input

fn input_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<String>().unwrap())
        .collect()
}

// - Instruction Transform

struct Instruction<'a> {
  movement: &'a str,
  distance: i32
}

fn instruct(i: &str) -> Instruction {
  let split: Vec<&str> = i.split(' ').collect();

  Instruction {movement: split[0], distance: split[1].parse::<i32>().unwrap()}
}

// Part 1

fn distance_depth_calc(input: &Vec<String>, mut h_pos: i32, mut d_pos: i32, position: usize) -> i32 {
  if input.len() == position { return h_pos * d_pos }

  let instruction = instruct(&input[position]);

  match instruction.movement {
    "forward" => h_pos += instruction.distance,
    "down"    => d_pos += instruction.distance,
    "up"      => d_pos -= instruction.distance,
    _         => println!("Unknown Direction Instruction!"),
  }

  distance_depth_calc(input, h_pos, d_pos, position + 1)
}

// Part 2

fn distance_depth_calc_rev2(input: &Vec<String>, mut h_pos: i32, mut d_pos: i32, mut aim: i32, position: usize) -> i32 {
  if input.len() == position { return h_pos * d_pos }

  let instruction = instruct(&input[position]);

  match instruction.movement {
    "forward" => {
      h_pos += instruction.distance;
      d_pos += aim * instruction.distance;
    },
    "down" => aim += instruction.distance,
    "up"   => aim -= instruction.distance,
    _      => println!("Unknown Direction Instruction!"),
  }

  distance_depth_calc_rev2(input, h_pos, d_pos, aim, position + 1)
}

// Main

fn main() {
    let input = input_file("./input.txt");
    println!("Distance + Depth Calc:      {:?}", distance_depth_calc(&input, 0, 0, 0));
    println!("Distance + Depth Calc Rev2: {:?}", distance_depth_calc_rev2(&input, 0, 0, 0, 0));
}

// Tests

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_distance_depth_calc() {
        let input = input_file("./sample.txt");
        assert_eq!(150, distance_depth_calc(&input, 0, 0, 0));
    }

    #[test]
    fn test_distance_depth_calc_rev2() {
        let input = input_file("./sample.txt");
        assert_eq!(900, distance_depth_calc_rev2(&input, 0, 0, 0, 0));
    }
}