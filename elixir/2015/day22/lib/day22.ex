defmodule Day22 do
  @moduledoc """
  Documentation for `Day22`.
  """

  def part1() do
    find_best()
  end

  def part2() do
    find_best(:hard)
  end

  def find_best(mode \\ :normal, spell_list \\ [[]], {c, best} \\ {1_000_000, nil}) do
    spell_lists = for s <- spell_list, a <- Spell.spells(), do: s ++ [a]
    spell_lists = Enum.filter(spell_lists, fn spell_list -> list_cost(spell_list) <= c end)

    Enum.reduce(spell_lists, {c, best}, fn spell_list, {cc, bb} ->
      cost = list_cost(spell_list)

      if cost do
        player_stats = %Stats{health: 50, mana: 500, spells: spell_list}
        boss_stats = %Stats{health: 55, mana: 0, damage: 8}
        battle_state = BattleSim.new_state(player_stats, boss_stats)
        result = BattleSim.battle(battle_state, mode)

        case result do
          :win ->
            {cost, spell_list}

          :loss_spells ->
            find_best(mode, [spell_list], {cc, bb})

          _ ->
            {cc, bb}
        end
      else
        {cc, bb}
      end
    end)
  end

  def list_cost(spell_list) do
    Enum.reduce(spell_list, 0, fn %Spell{mana: m}, acc -> acc + m end)
  end
end
