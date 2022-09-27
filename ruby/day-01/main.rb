#!/usr/bin/env ruby

require './puzzle'

module Aoc
  module Submarine
    module Sonar

      def Sonar.two_measure_diff_count(readings)
        readings
          .each_cons(2)
          .map {|a,b| a < b }
          .count(true)
      end

      def Sonar.three_measure_diff_count(readings)
        two_measure_diff_count(
          readings
            .each_cons(3)
            .map {|c| c.inject(0, :+) }
        )
      end

    end
  end
end

require 'rspec/autorun'

RSpec.configure do |config|

 config.tty       = true
 config.formatter = :documentation 

end

RSpec.describe 'Sonar' do

  let(:puzzle) { Aoc::Puzzle.new }

  it '#two_measure_diff_count' do
    expect(Aoc::Submarine::Sonar.two_measure_diff_count(puzzle.sample)).to be(7)
  end

  it '#three_measure_diff_count' do
    expect(Aoc::Submarine::Sonar.three_measure_diff_count(puzzle.sample)).to be(5)
  end

end

puzzle = Aoc::Puzzle.new

p Aoc::Submarine::Sonar.two_measure_diff_count(puzzle.input)
p Aoc::Submarine::Sonar.three_measure_diff_count(puzzle.input)