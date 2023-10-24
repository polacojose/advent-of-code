defmodule Day22Test do
  use ExUnit.Case
  doctest Day22

  # test "Demo Battle Win" do
  #   spells =
  #     Enum.flat_map([:poison, :magic_missle], fn t ->
  #       for %{type: ^t} = s <- Spell.spells(), do: s
  #     end)

  #   player_stats = %PlayerStats{health: 10, mana: 250, spells: spells}
  #   boss_stats = %PlayerStats{health: 13, damage: 8}

  #   battle_state = BattleSim.new_state(player_stats, boss_stats)

  #   assert(BattleSim.battle(battle_state) == :win)
  # end

  # test "Demo Battle Long Win" do
  #   spells =
  #     Enum.flat_map([:recharge, :shield, :drain, :poison, :magic_missle], fn t ->
  #       for %{type: ^t} = s <- Spell.spells(), do: s
  #     end)

  #   player_stats = %PlayerStats{health: 10, mana: 250, spells: spells}
  #   boss_stats = %PlayerStats{health: 14, damage: 8}

  #   battle_state = BattleSim.new_state(player_stats, boss_stats)

  #   assert(BattleSim.battle(battle_state) == :win)
  # end

  test "Part 1" do
    spells =
      [
        :magic_missle,
        :poison,
        :recharge,
        :magic_missle,
        :shield,
        :poison,
        :magic_missle,
        :magic_missle,
        :magic_missle
      ]

    spells =
      Enum.flat_map(spells, fn t ->
        for %{type: ^t} = s <- Spell.spells(), do: s
      end)

    IO.inspect(spells)

    player_stats = %PlayerStats{health: 50, mana: 500, spells: spells}
    boss_stats = %PlayerStats{health: 55, damage: 8}

    battle_state = BattleSim.new_state(player_stats, boss_stats)

    assert(BattleSim.battle(battle_state) == :win)
  end
end
