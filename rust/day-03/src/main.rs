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

// - Auto Mask

fn automask(diag_len: usize) -> u16 {
  let padding:    Vec<u8> = vec![1;16 - diag_len];
  let diagnostic: Vec<u8> = vec![0;diag_len];
  let mask:       Vec<u8> =[padding, diagnostic].concat();

  binary_set_to_int(mask)
}

// - Binary String to u16

fn binary_set_to_int(binary_set: Vec<u8>) -> u16 {
  let gamma_string: String = binary_set.iter().map(|t| t.to_string()).collect::<String>();

  u16::from_str_radix(&gamma_string, 2).unwrap()
}

// - Common Bits

fn common_bits(tally: Vec<u16>, length: u16) -> Vec<u8> {
  tally.iter().map(|t| (t >= &( length - t )) as u8).collect()
}

// - UnCommon Bits

fn uncommon_bits(tally: Vec<u16>, length: u16) -> Vec<u8> {
  tally.iter().map(|t| (t < &( length - t )) as u8).collect()
}


// - Bit Accumulator

fn bit_accumulator(input: &Vec<String>, mut tally: Vec<u16>, position: usize) -> Vec<u16> {
  if input.len() == position { return tally };

  let mut ch_pos: usize = 0;

  for c in input[position].chars() {
    if c.to_string() == "1" { tally[ch_pos] += 1 }
    ch_pos += 1
  }

  bit_accumulator(input, tally, position + 1)
}

fn bit_set_filter(input: &Vec<String>, position: usize, common: bool) -> Vec<u8> {
  if input.len() == 1 as usize {
    return input[0].chars().map(|b|
      b.to_digit(10).unwrap() as u8
    ).collect();
  };

  let tally:  Vec<u16> = bit_accumulator(input, vec![0; input[0].len()], 0);
  let filter: Vec<u8>  = if common {
    common_bits(tally, input.len() as u16 )
  } else {
    uncommon_bits(tally, input.len() as u16 )
  };

  let remaining: Vec<String> = input.iter().filter(|b_string|
    b_string.chars().nth(position).unwrap().to_digit(10).unwrap() as u8 == filter[position]
  ).cloned().collect();

  bit_set_filter(&remaining, position + 1, common)
}


// Part 1

fn power_consumption_rev1(input: &Vec<String>) -> u32 {
  let tally: Vec<u16> = bit_accumulator(input, vec![0; input[0].len()], 0);
  let bits:  Vec<u8>  = common_bits(tally, input.len() as u16 );

  let gamma: u16 = binary_set_to_int(bits);
  let mask:  u16 = automask(input[0].len());

  gamma as u32 * ( !gamma ^ mask ) as u32
}

// Part 2

fn power_consumption_rev2(input: &Vec<String>) -> u32 {
  let oxy: u16 = binary_set_to_int(bit_set_filter(input, 0, true));
  let co2: u16 = binary_set_to_int(bit_set_filter(input, 0, false));

  oxy as u32 * co2 as u32
}

// Main

fn main() {
  let input = input_file("./input.txt");

  println!("Power Consumption Rev1: {:?}", power_consumption_rev1(&input));
  println!("Power Consumption Rev2: {:?}", power_consumption_rev2(&input));
}

// Tests

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_power_consumption_rev1() {
        let input = input_file("./sample.txt");
        assert_eq!(198, power_consumption_rev1(&input));
    }

    #[test]
    fn test_power_consumption_rev2() {
        let input = input_file("./sample.txt");
        assert_eq!(230, power_consumption_rev2(&input));
    }
}