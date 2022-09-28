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

struct PuzzelInput {
  drawings: Vec<u8>,
  puzzels:  Vec<Vec<u8>>
}

fn puzzel_inputs(
    input: &Vec<String>,
    drawings: Vec<u8>,
    puzzels: mut Vec<Vec<u8>>,
    p_pos: usize,
    position: usize
  ) -> PuzzelInput {
  if input.len() == position { return PuzzelInput{ drawings: drawings, puzzels: puzzels } };

  if position < 1 {
    let drawings: Vec<u8> = input[position].clone().split(",").map(|d| d.parse().unwrap() ).collect();
  } else {
    println!("{:?}", input[position])
  }

    puzzel_inputs(input, drawings, puzzels, p_pos, position + 1)
}

// Part 1

// fn power_consumption_rev1(input: &Vec<String>) -> u32 {
//   let tally: Vec<u16> = bit_accumulator(input, vec![0; input[0].len()], 0);
//   let bits:  Vec<u8>  = common_bits(tally, input.len() as u16 );

//   let gamma: u16 = binary_set_to_int(bits);
//   let mask:  u16 = automask(input[0].len());

//   gamma as u32 * ( !gamma ^ mask ) as u32
// }

// Part 2

// fn power_consumption_rev2(input: &Vec<String>) -> u32 {
//   let oxy: u16 = binary_set_to_int(bit_set_filter(input, 0, true));
//   let co2: u16 = binary_set_to_int(bit_set_filter(input, 0, false));

//   oxy as u32 * co2 as u32
// }

// Main

fn main() {
  let input = input_file("./sample.txt");

  println!("{:?}", puzzel_inputs(&input, Vec::new();));

  // println!("Power Consumption Rev1: {:?}", power_consumption_rev1(&input));
  // println!("Power Consumption Rev2: {:?}", power_consumption_rev2(&input));
}

// Tests

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_power_consumption_rev1() {
//         let input = input_file("./sample.txt");
//         assert_eq!(198, power_consumption_rev1(&input));
//     }

//     #[test]
//     fn test_power_consumption_rev2() {
//         let input = input_file("./sample.txt");
//         assert_eq!(230, power_consumption_rev2(&input));
//     }
// }