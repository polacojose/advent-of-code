defmodule NativeVariantTest do
  use ExUnit.Case
  doctest NativeVariant

  test "turn on part 1" do
    insts = [NativeVariant.parse_instruction("turn on 0,0 through 999,999")]
    grid = NativeVariant.new_lights_part1(1000, 1000, false)
    assert(NativeVariant.how_many_lights?(insts, grid) == 1_000_000)
  end

  test "turn toggle part 1" do
    insts = [NativeVariant.parse_instruction("toggle 0,0 through 999,0")]
    grid = NativeVariant.new_lights_part1(1000, 1000, false)
    assert(NativeVariant.how_many_lights?(insts, grid) == 1000)
  end

  test "turn off part 1" do
    insts = [NativeVariant.parse_instruction("turn off 499,499 through 500,500")]
    grid = NativeVariant.new_lights_part1(1000, 1000, true)
    assert(NativeVariant.how_many_lights?(insts, grid) == 999_996)
  end

  test "turn on part 2" do
    insts = [NativeVariant.parse_instruction("turn on 0,0 through 0,0")]
    grid = NativeVariant.new_lights_part2(1000, 1000, 0)
    assert(NativeVariant.how_bright?(insts, grid) == 1)
  end

  test "turn toggle part 2" do
    insts = [NativeVariant.parse_instruction("toggle 0,0 through 999,999")]
    grid = NativeVariant.new_lights_part2(1000, 1000, 0)
    assert(NativeVariant.how_bright?(insts, grid) == 2_000_000)
  end

  test "turn off part 2" do
    insts = [NativeVariant.parse_instruction("turn off 499,499 through 500,500")]
    grid = NativeVariant.new_lights_part2(1000, 1000, 1)
    assert(NativeVariant.how_bright?(insts, grid) == 999_996)
  end
end
