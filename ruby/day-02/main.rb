#!/usr/bin/env ruby

require './puzzle'

module Aoc
  module Submarine
    class Position
        
      def initialize(horizontal: 0, vertical: 0)
        @horizontal = horizontal
        @vertical   = vertical
      end

      def forward(u) @horizontal += u end
      def down(u)    @vertical   += u end
      def up(u)      @vertical   -= u end

      def current() @horizontal * @vertical end

    end

    class Position_W_Aim
      
      def initialize(horizontal: 0, vertical: 0, aim: 0)
        @horizontal = horizontal
        @vertical   = vertical
        @aim        = aim
      end

      def forward(u)
        @horizontal += u
        @vertical   += u * @aim
      end

      def down(u) @aim += u end
      def up(u)   @aim -= u end

      def current() @horizontal * @vertical end

    end

    class Controller

      def initialize(commands: [])
        @commands = commands
      end

      def process(position)
        @commands.each do |c|
          command, unit = c.split(' ')
          position.send(command.to_sym, unit.to_i)
        end
        position.current
      end

    end
  end
end

require 'rspec/autorun'

RSpec.configure do |config|

 config.tty       = true
 config.formatter = :documentation 

end

RSpec.describe 'Controller' do

  let(:puzzle)     { Aoc::Puzzle.sample }
  let(:controller) { Aoc::Submarine::Controller.new(commands: puzzle) }
  let(:position)   { controller.process(Aoc::Submarine::Position.new(horizontal: 0, vertical: 0)) }

  let(:position_w_aim) { controller.process(Aoc::Submarine::Position_W_Aim.new(horizontal: 0, vertical: 0, aim: 0)) }

  it '#process' do
    printf("The Sample Puzzle Answer is: %d\n", position)
    expect(position).to be(150)
  end

  it '#process_w_aim' do
    printf("The Sample Puzzle Answer is: %d\n", position_w_aim)
    expect(position_w_aim).to be(900)
  end

end

controller = Aoc::Submarine::Controller.new(commands: Aoc::Puzzle.input)
position   = Aoc::Submarine::Position.new(horizontal: 0, vertical: 0)
printf("The Part 1 - Puzzle Answer is: %d\n", controller.process(position))

controller = Aoc::Submarine::Controller.new(commands: Aoc::Puzzle.input)
position   = Aoc::Submarine::Position_W_Aim.new(horizontal: 0, vertical: 0, aim: 0)
printf("The Part 2 - Puzzle Answer is: %d\n", controller.process(position))