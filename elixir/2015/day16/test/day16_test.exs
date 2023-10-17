defmodule Day16Test do
  use ExUnit.Case
  doctest Day16

  test "parses aunt" do
    assert Aunt.parse("Sue 69: vizslas: 5, perfumes: 3, cars: 9") == %{
             id: 69,
             vizslas: 5,
             perfumes: 3,
             cars: 9
           }
  end

  test "part1 outcome" do
    assert Day16.part1_which_aunt?().id == 40
  end

  test "part2 outcome" do
    assert Day16.part2_which_aunt?().id == 241
  end
end
