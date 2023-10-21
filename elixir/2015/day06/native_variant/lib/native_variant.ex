defmodule NativeVariant do
  @moduledoc """
  Documentation for `NativeVariant`.
  """
  use Rustler, otp_app: :native_variant, crate: "aoc_lights"

  @input "input.txt"
  def part1() do
    File.stream!(@input)
    |> Enum.map(&parse_instruction/1)
    |> how_many_lights?(new_lights_part1(1000, 1000, false))
  end

  def part2() do
    File.stream!(@input)
    |> Enum.map(&parse_instruction/1)
    |> how_bright?(new_lights_part2(1000, 1000, 0))
  end

  @spec how_many_lights?(list(), map()) :: integer
  def how_many_lights?(instructions, grid) do
    Enum.reduce(instructions, grid, fn inst, grid ->
      execute_instruction_part1(grid, inst)
    end)
    |> grid_content_part1()
    |> List.flatten()
    |> Enum.count(& &1)
  end

  @spec how_many_lights?(list(), map()) :: integer
  def how_bright?(instructions, grid) do
    Enum.reduce(instructions, grid, fn inst, grid ->
      execute_instruction_part2(grid, inst)
    end)
    |> grid_content_part2()
    |> List.flatten()
    |> Enum.sum()
  end

  def new_lights_part1(_arg1, _arg2, _arg3), do: :erlang.nif_error(:nif_not_loaded)
  def new_lights_part2(_arg1, _arg2, _arg3), do: :erlang.nif_error(:nif_not_loaded)
  def grid_content_part1(_arg1), do: :erlang.nif_error(:nif_not_loaded)
  def grid_content_part2(_arg1), do: :erlang.nif_error(:nif_not_loaded)
  def parse_instruction(_arg1), do: :erlang.nif_error(:nif_not_loaded)
  def execute_instruction_part1(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
  def execute_instruction_part2(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
end
