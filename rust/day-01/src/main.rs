use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

// Input

fn input_file(filename: impl AsRef<Path>) -> Vec<i16> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<i16>().unwrap())
        .collect()
}

// Part 1

fn depth_forcast(input: &Vec<i16>) -> i16 {
  let mut forcast:  i16 = 0;
  let mut previous: i16 = i16::MAX;
  for depth in input {
    if depth > &previous {
      forcast += 1
    }
    previous = *depth
  }
  forcast
}

fn recurs_depth_forcast(input: &Vec<i16>, mut forcast: i16, previous: i16, position: usize) -> i16 {
  if input[position] > previous { forcast += 1 }
  if input.len() == position + 1 { return forcast }
  recurs_depth_forcast(input, forcast, input[position], position + 1)
}

// Part 2

fn depth_forcast_sliding(input: &Vec<i16>) -> i16 {
  let mut forcast:  i16   = 0;
  let mut previous: i16   = i16::MAX;
  let     ending:   usize = input.len() - 2;
  for step in 0..ending {
    let slice_sum: i16 = input[step..step + 3].iter().sum::<i16>();
    if slice_sum > previous {
      forcast += 1
    }
    previous = slice_sum
  }
  forcast
}

fn recurs_depth_forcast_sliding(input: &Vec<i16>, mut forcast: i16, previous: i16, position: usize) -> i16 {
  let slice_sum: i16 = input[position..position + 3].iter().sum::<i16>();
  if slice_sum > previous { forcast += 1 }
  if input.len() == position + 3 { return forcast }
  recurs_depth_forcast_sliding(input, forcast, slice_sum, position + 1)
}

// Main

fn main() {
    let input = input_file("./input.txt");
    println!("Depth Forcast:                   {:?}", depth_forcast(&input));
    println!("Recursive Depth Forcast:         {:?}", recurs_depth_forcast(&input, 0, i16::MAX, 0));
    println!("Depth Forcast Sliding:           {:?}", depth_forcast_sliding(&input));
    println!("Recursive Depth Forcast Sliding: {:?}", recurs_depth_forcast_sliding(&input, 0, i16::MAX, 0));
}

// Tests

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const EXAMPLE_INPUT: [i16; 10] = [ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 ];

    #[test]
    fn test_depth_forcast() {
        assert_eq!(7, depth_forcast(&EXAMPLE_INPUT.to_vec()));
    }

    #[test]
    fn test_depth_forcast_sliding() {
        assert_eq!(5, depth_forcast_sliding(&EXAMPLE_INPUT.to_vec()));
    }
}