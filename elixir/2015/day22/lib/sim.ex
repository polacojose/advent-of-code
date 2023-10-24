defmodule BattleSim do
  @moduledoc """
  Simulates the battle
  """
  defstruct effects: [], player: nil, boss: nil
  @type state :: %__MODULE__{}
  @type phase :: :player | :boss

  @spec new_state(%PlayerStats{}, %PlayerStats{}) :: %__MODULE__{}
  def new_state(player, boss) do
    %__MODULE__{player: player, boss: boss}
  end

  @spec battle(%__MODULE__{}) :: :win | :loss
  def battle(state \\ %__MODULE__{}, phase \\ :player)
  def battle(%__MODULE__{boss: %PlayerStats{health: h}}, _) when h <= 0, do: :win
  def battle(%__MODULE__{player: %PlayerStats{health: h}}, :player) when h <= 0, do: :loss
  def battle(%__MODULE__{player: %PlayerStats{mana: m}}, :player) when m <= 0, do: :loss
  def battle(%__MODULE__{player: %PlayerStats{spells: []}}, :player), do: :loss

  # Player Phase
  def battle(state, :player) do
    # %__MODULE__{player: player, boss: boss} = state

    # IO.puts("""
    # -- Player turn --
    # - Player has #{player.health} hit points, #{player.armor} armor, #{player.mana} mana
    # - Boss has #{boss.health} hit points
    # """)

    state = resolve_effects(state)
    state = cast_spell(state)
    battle(state, :boss)
  end

  # Boss Phase
  def battle(state, :boss) do
    # %__MODULE__{player: player, boss: boss} = state

    # IO.puts("""
    # -- Boss turn --
    # - Player has #{player.health} hit points, #{player.armor} armor, #{player.mana} mana
    # - Boss has #{boss.health} hit points
    # """)

    state = resolve_effects(state)
    %__MODULE__{player: player, boss: boss} = state
    attack = max(boss.damage - player.armor, 1)
    player = %{player | health: player.health - attack}

    # IO.puts("Boss deals #{attack} damage.")

    battle(%{state | player: player}, :player)
  end

  @spec cast_spell(%__MODULE__{}) :: %__MODULE__{}
  def cast_spell(state) do
    %__MODULE__{player: player, boss: boss, effects: effects} = state
    spell = player.spells |> hd
    player = %{player | spells: player.spells |> tl}

    case spell.type do
      :magic_missle ->
        player = %{player | mana: player.mana - 53}
        boss = %{boss | health: boss.health - 4}

        # IO.puts("Player casts Magic Missile dealing 4 damage.")

        state = %{state | player: player}
        state = %{state | boss: boss}
        %{state | effects: effects}

      :drain ->
        player = %{player | mana: player.mana - 73}
        boss = %{boss | health: boss.health - 2}
        player = %{player | health: player.health + 2}

        # IO.puts("Player casts Drain, dealing 2 damage, and healing 2 hit points.")

        state = %{state | player: player}
        state = %{state | boss: boss}
        %{state | effects: effects}

      :shield ->
        player = %{player | mana: player.mana - 113}
        state = %{state | player: player}

        if !Enum.any?(effects, fn %{type: t} -> t == spell.type end) do
          player = %{player | armor: player.armor + 7}
          effects = [spell | effects]

          # IO.puts("Player casts Shield, increasing armor by 7.")

          state = %{state | player: player}
          state = %{state | boss: boss}
          %{state | effects: effects}
        else
          state
        end

      :poison ->
        player = %{player | mana: player.mana - 173}
        state = %{state | player: player}

        if !Enum.any?(effects, fn %{type: t} -> t == spell.type end) do
          effects = [spell | effects]

          # IO.puts("Player casts Poison.")

          state = %{state | player: player}
          state = %{state | boss: boss}
          %{state | effects: effects}
        else
          state
        end

      :recharge ->
        player = %{player | mana: player.mana - 229}
        state = %{state | player: player}

        if !Enum.any?(effects, fn %{type: t} -> t == spell.type end) do
          effects = [spell | effects]

          # IO.puts("Player casts Recharge.")

          state = %{state | player: player}
          state = %{state | boss: boss}
          %{state | effects: effects}
        else
          state
        end
    end
  end

  @spec resolve_effects(%__MODULE__{}) :: %__MODULE__{}
  def resolve_effects(state) do
    %__MODULE__{player: player, boss: boss, effects: effects} = state
    state = %{state | effects: []}

    Enum.reduce(effects, state, fn spell, state ->
      case spell.type do
        :shield ->
          spell = %{spell | turns: spell.turns - 1}

          # IO.puts("Shields timer is now #{spell.turns}." |> String.capitalize())

          if spell.turns > 0 do
            effects = [spell | state.effects]
            %{state | effects: effects}
          else
            player = %{player | armor: player.armor - 7}
            %{state | player: player}
          end

        :poison ->
          spell = %{spell | turns: spell.turns - 1}
          boss = %{boss | health: boss.health - 3}
          state = %{state | boss: boss}

          # IO.puts(
          #  "#{spell.type} deals 3 damage; it's timer is now #{spell.turns}."
          #  |> String.capitalize()
          # )

          if spell.turns > 0 do
            effects = [spell | state.effects]
            %{state | effects: effects}
          else
            state
          end

        :recharge ->
          spell = %{spell | turns: spell.turns - 1}
          player = %{player | mana: player.mana + 101}
          state = %{state | player: player}

          if spell.turns > 0 do
            effects = [spell | state.effects]
            %{state | effects: effects}
          else
            state
          end
      end
    end)
  end
end
