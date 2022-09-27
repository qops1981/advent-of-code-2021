module Aoc
    class Puzzle
      attr_reader :input_file
  
      def initialize()
        @input_file = Dir.glob("./**/input.txt").first
      end
  
      def input() File.read(@input_file).split("\n").map {|s| s.to_i} end
  
      def sample() [199,200,208,210,200,207,240,269,260,263] end
    end
end