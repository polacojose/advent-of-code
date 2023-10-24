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
    spells = [
      %Spell{type: :shield, mana: 113, turns: 6},
      %Spell{type: :recharge, mana: 229, turns: 5},
      %Spell{type: :poison, mana: 173, turns: 6},
      %Spell{type: :magic_missle, mana: 53, turns: 0},
      %Spell{type: :magic_missle, mana: 53, turns: 0},
      %Spell{type: :poison, mana: 173, turns: 6},
      %Spell{type: :magic_missle, mana: 53, turns: 0},
      %Spell{type: :magic_missle, mana: 53, turns: 0},
      %Spell{type: :magic_missle, mana: 53, turns: 0}
    ]

    IO.inspect(spells)

    player_stats = %Stats{health: 50, mana: 500, spells: spells}
    boss_stats = %Stats{health: 55, damage: 8}

    battle_state = BattleSim.new_state(player_stats, boss_stats)

    result = BattleSim.battle(battle_state, :hard)
    assert result != :win
  end
end
