module Aoc
    module Puzzle
  
      def Puzzle.input() load("./input.txt") end
  
      def Puzzle.sample() load("./sample.txt") end

      def Puzzle.load(f) File.read(f).split("\n") end

    end
end