defmodule Day21Test do
  use ExUnit.Case
  doctest Day21

  test "Demo Win" do
    player = %PlayerStats{health: 8, damage: 5, armor: 5}
    boss = %PlayerStats{health: 12, damage: 7, armor: 2}
    assert(BattleSim.battle(player, boss) == :win)
  end

  test "Solutions" do
    assert(Day21.part1() |> elem(0) == 91)
    assert(Day21.part2() |> elem(0) == 158)
  end
end
