defmodule Day21 do
  @moduledoc """
  Documentation for `Day21`.
  """

  @spec part1() :: {non_neg_integer(), [%Item{}]}
  def part1() do
    PlayerStats.player_loadouts()
    |> PlayerStats.sort_loadouts_with_cost()
    |> Enum.find(fn {_, lo} ->
      player = PlayerStats.from_loadout(lo, 100)
      boss = %PlayerStats{health: 100, damage: 8, armor: 2}
      BattleSim.battle(player, boss) == :win
    end)
  end

  @spec part2() :: {non_neg_integer(), [%Item{}]}
  def part2() do
    PlayerStats.player_loadouts()
    |> PlayerStats.sort_loadouts_with_cost(:desc)
    |> Enum.find(fn {_, lo} ->
      player = PlayerStats.from_loadout(lo, 100)
      boss = %PlayerStats{health: 100, damage: 8, armor: 2}
      BattleSim.battle(player, boss) == :loss
    end)
  end
end
