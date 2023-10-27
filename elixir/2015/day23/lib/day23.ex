defmodule Day23 do
  @moduledoc """
  Documentation for `Day23`.
  """

  def part1() do
    instructions = Instruction.parse_instructions("input.txt")
    {:ok, t} = Turing.new()
    Turing.execute_program(t, instructions)
    Turing.registers(t)
  end

  def part2() do
    instructions = Instruction.parse_instructions("input.txt")
    instructions = [Instruction.parse_instruction("inc a") | instructions]
    {:ok, t} = Turing.new()
    Turing.execute_program(t, instructions)
    Turing.registers(t)
  end
end
