defmodule Day22 do
  @moduledoc """
  Documentation for `Day22`.
  """

  def part1(i \\ 1, {min, last} \\ {1_000_000, nil}) do
    IO.inspect([i, min, last])

    spell_lists =
      Spell.spell_lists(i)
      |> Enum.map(fn x ->
        {Enum.reduce(x, 0, fn %Spell{mana: m}, acc -> acc + m end), x}
      end)
      |> Enum.filter(fn {c, _} -> c < min end)

    if Enum.empty?(spell_lists) do
      {min, last}
    else
      wins =
        Enum.filter(spell_lists, fn {_, spells} ->
          player_stats = %PlayerStats{health: 50, mana: 500, spells: spells}
          boss_stats = %PlayerStats{health: 55, mana: 0, damage: 8}
          battle_state = BattleSim.new_state(player_stats, boss_stats)
          BattleSim.battle(battle_state) == :win
        end)

      case length(wins) do
        0 ->
          part1(i + 1, {min, last})

        _ ->
          last_best =
            Enum.filter(wins, fn {c, _} -> c < min end)
            |> Enum.sort_by(fn {c, _} -> c end)
            |> List.first()

          case last_best do
            {c, _} ->
              if c < min do
                part1(i + 1, last_best)
              end

            _ ->
              part1(i + 1, {min, last})
          end
      end
    end
  end
end
