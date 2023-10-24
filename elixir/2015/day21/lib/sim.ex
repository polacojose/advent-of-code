defmodule BattleSim do
  @moduledoc """
  Simulates the battle
  """

  @type phase :: :player | :boss

  @spec battle(%PlayerStats{}, %PlayerStats{}) :: :win | :loss

  def battle(player, boss, phase \\ :player)

  def battle(_, %PlayerStats{health: h}, _) when h <= 0, do: :win
  def battle(%PlayerStats{health: h}, _, _) when h <= 0, do: :loss

  def battle(player, boss, :player) do
    attack = max(player.damage - boss.armor, 1)
    battle(player, %{boss | health: boss.health - attack}, :boss)
  end

  def battle(player, boss, :boss) do
    attack = max(boss.damage - player.armor, 1)
    battle(%{player | health: player.health - attack}, boss, :player)
  end
end
