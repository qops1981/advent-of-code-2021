module Aoc
  module Puzzle

    def Puzzle.input() load("./input.txt").map {|s| s.to_i} end

    def Puzzle.sample() load("./sample.txt").map {|s| s.to_i} end

    def Puzzle.load(f) File.read(f).split("\n") end
        
  end
end